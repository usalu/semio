//! 🧪️ Standalone scratch crate: DWG R2004+ (AC1018+) file-structure primitives, proven in
//! isolation against the REAL 145KB `architectural.dwg` fixture (ticket 26/08/10/
//! ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, 🖊️dwg D1-D2 wave) before being
//! ported into the actual stdio crate. Algorithm cross-checked against the ODA-spec-derived
//! LibreDWG reference implementation (GPLv3, github.com/LibreDWG/libredwg `src/decode.c` +
//! `src/r2004_file_header.spec`) for field layout/opcode semantics only -- this is a clean-room
//! reimplementation, no code copied.
//!
//! Found one real bug during this validation: `two_byte_offset`'s pre-existing partial
//! `comp_offset` bits (set by the caller for opcode1 in 0x10-0x1F) must be OR'd together with the
//! two new offset bytes BEFORE `+= plus` is applied to the combined value -- applying `plus` to
//! only the two new bytes and OR-ing the pre-existing bits in afterward silently desyncs the
//! decompressor on real (longer, more opcode-varied) section data while still "succeeding" on
//! shorter streams. Exactly the class of bug this technique exists to catch.

use std::fs;

//#region FileHeaderDecrypt
/// 🔓 R2004+ file header "decryption" -- not real security, a fixed LCG-generated one-time pad
/// (the classic Borland/MSVC `rand()` constants: `seed = seed*0x343fd + 0x269ec3`, upper 16 bits
/// of `seed` XORed per byte). Symmetric (same fn encrypts and decrypts).
fn decrypt_r2004_header(src: &[u8]) -> Vec<u8> {
    let mut rseed: u32 = 1;
    src.iter()
        .map(|&b| {
            rseed = rseed.wrapping_mul(0x343fd).wrapping_add(0x269ec3);
            b ^ ((rseed >> 0x10) & 0xFF) as u8
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct R2004FileHeader {
    file_id_string: [u8; 12],
    last_section_id: u32,
    last_section_address: u64,
    numgaps: u32,
    numsections: u32,
    section_map_id: u32,
    section_map_address: u64,
    section_info_id: i32,
    section_array_size: u32,
}

fn parse_r2004_file_header(dec: &[u8]) -> R2004FileHeader {
    let u32_at = |o: usize| u32::from_le_bytes(dec[o..o + 4].try_into().unwrap());
    let u64_at = |o: usize| u64::from_le_bytes(dec[o..o + 8].try_into().unwrap());
    let mut file_id_string = [0u8; 12];
    file_id_string.copy_from_slice(&dec[0..12]);
    R2004FileHeader {
        file_id_string,
        last_section_id: u32_at(0x28),
        last_section_address: u64_at(0x2c),
        numgaps: u32_at(0x3c),
        numsections: u32_at(0x40),
        section_map_id: u32_at(0x50),
        section_map_address: u64_at(0x54),
        section_info_id: u32_at(0x5c) as i32,
        section_array_size: u32_at(0x60),
    }
}
//#endregion FileHeaderDecrypt

//#region Lz77Variant
/// 🗜️ R2004+ "compression algorithm 2": a bespoke byte-oriented LZ77 variant (NOT DEFLATE).
/// Opcode stream of interleaved match (back-reference) and literal-copy runs, terminated by
/// opcode byte `0x11` or source exhaustion. `decomp_size` upper-bounds the output buffer (real
/// readers use each section's generous `max_decomp_size` allocation, not a tight fit -- the
/// meaningful content ends wherever the terminator naturally falls).
struct ByteCursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> ByteCursor<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }
    fn u8(&mut self) -> Result<u8, String> {
        let b = *self.data.get(self.pos).ok_or("dwg lz: source exhausted")?;
        self.pos += 1;
        Ok(b)
    }
    fn has_more(&self) -> bool {
        self.pos < self.data.len()
    }
}

fn read_literal_length(src: &mut ByteCursor, opcode: u8) -> Result<u32, String> {
    let mut lowbits = (opcode & 0xF) as u32;
    if lowbits == 0 {
        let mut lastbyte;
        loop {
            lastbyte = src.u8()?;
            if lastbyte != 0 || !src.has_more() {
                break;
            }
            lowbits += 0xFF;
        }
        lowbits += 0xF + lastbyte as u32;
    }
    Ok(lowbits + 3)
}

fn read_compressed_bytes(src: &mut ByteCursor, opcode: u8, bits: u32) -> Result<u32, String> {
    let mut cb = (opcode as u32) & bits;
    if cb == 0 {
        let mut lastbyte;
        loop {
            lastbyte = src.u8()?;
            if lastbyte != 0 || !src.has_more() {
                break;
            }
            cb += 0xFF;
        }
        cb += lastbyte as u32 + bits;
    }
    Ok(cb + 2)
}

/// ⚠️ `existing_offset` carries bits the caller already set (opcode1 in 0x10-0x1F stashes
/// `(opcode1&8)<<11` before calling this) -- they must be OR'd into the combined offset BEFORE
/// `plus` is added, not added separately afterward (see module docs: this was a real bug).
fn two_byte_offset(src: &mut ByteCursor, plus: u32, existing_offset: u32) -> Result<(u8, u32), String> {
    let first = src.u8()?;
    let second = src.u8()?;
    let offset = existing_offset | ((first as u32) >> 2) | ((second as u32) << 6);
    Ok((first, offset + plus))
}

fn copy_bytes(n: u32, src: &mut ByteCursor, dec: &mut Vec<u8>) -> Result<u8, String> {
    for _ in 0..n {
        dec.push(src.u8()?);
    }
    src.u8()
}

fn decompress_r2004_section(comp: &[u8], decomp_size: usize) -> Result<Vec<u8>, String> {
    let mut src = ByteCursor::new(comp);
    let mut dec: Vec<u8> = Vec::with_capacity(decomp_size.min(1 << 20));

    let mut opcode1 = src.u8()?;
    if (opcode1 & 0xF0) == 0 {
        let lit_len = read_literal_length(&mut src, opcode1)?;
        opcode1 = copy_bytes(lit_len, &mut src, &mut dec)?;
    }

    while src.has_more() && dec.len() < decomp_size && opcode1 != 0x11 {
        let (comp_bytes, comp_offset): (u32, u32);
        if opcode1 < 0x10 || opcode1 >= 0x40 {
            let cb = ((opcode1 as u32) >> 4).wrapping_sub(1);
            let opcode2 = src.u8()?;
            let co = ((((opcode1 as u32) >> 2) & 3) | ((opcode2 as u32) << 2)) + 1;
            comp_bytes = cb;
            comp_offset = co;
            // opcode1 unchanged for this branch's continuation semantics (matches reference).
        } else if opcode1 < 0x20 {
            let cb = read_compressed_bytes(&mut src, opcode1, 7)?;
            let partial = ((opcode1 as u32) & 8) << 11;
            let (next_op, co) = two_byte_offset(&mut src, 0x4000, partial)?;
            opcode1 = next_op;
            comp_bytes = cb;
            comp_offset = co;
        } else {
            debug_assert!(opcode1 >= 0x20);
            let cb = read_compressed_bytes(&mut src, opcode1, 0x1f)?;
            let (next_op, co) = two_byte_offset(&mut src, 1, 0)?;
            opcode1 = next_op;
            comp_bytes = cb;
            comp_offset = co;
        }

        let pos = dec.len();
        let end = pos + comp_bytes as usize;
        if end > decomp_size || (comp_offset as usize) > pos {
            return Err(format!(
                "dwg lz: invalid backref bytes={comp_bytes} offset={comp_offset} pos={pos} decomp_size={decomp_size}"
            ));
        }
        for i in pos..end {
            let b = dec[i - comp_offset as usize];
            dec.push(b);
        }

        let mut lit_length = (opcode1 & 3) as u32;
        if lit_length == 0 {
            opcode1 = src.u8()?;
            if (opcode1 & 0xF0) == 0 {
                lit_length = read_literal_length(&mut src, opcode1)?;
            }
        }
        if lit_length > 0 && end + lit_length as usize <= decomp_size {
            opcode1 = copy_bytes(lit_length, &mut src, &mut dec)?;
        } else if lit_length > 0 {
            break;
        }
    }
    Ok(dec)
}
//#endregion Lz77Variant

//#region PageHeader
/// 🔓 Per-named-section-page 32-byte encrypted header: `word[k] ^= (0x4164536b ^ file_address)`,
/// all little-endian u32 words. `page_type` must equal `0x4163043b` once decrypted -- the one
/// self-checking invariant every real reader validates before trusting the rest of the header.
#[derive(Debug)]
struct PageHeader {
    page_type: u32,
    section_type: u32,
    data_size: u32,
    page_size: u32,
    address_offset: u32,
}

fn decrypt_page_header(raw32: &[u8; 32], file_address: usize) -> PageHeader {
    let mask = 0x4164536bu32 ^ (file_address as u32);
    let mut words = [0u32; 8];
    for k in 0..8 {
        let w = u32::from_le_bytes(raw32[k * 4..k * 4 + 4].try_into().unwrap());
        words[k] = w ^ mask;
    }
    PageHeader {
        page_type: words[0],
        section_type: words[1],
        data_size: words[2],
        page_size: words[3],
        address_offset: words[4],
    }
}
//#endregion PageHeader

//#region SectionMapAndInfo
#[derive(Debug, Clone)]
struct PageDirEntry {
    number: i32,
    size: u32,
    address: u64,
}

fn parse_page_directory(dec: &[u8], section_array_size: u32) -> Vec<PageDirEntry> {
    let mut out = Vec::new();
    let mut pos = 0usize;
    let mut address: u64 = 0x100;
    while pos + 8 <= dec.len() {
        let number = i32::from_le_bytes(dec[pos..pos + 4].try_into().unwrap());
        let size = u32::from_le_bytes(dec[pos + 4..pos + 8].try_into().unwrap());
        pos += 8;
        let entry = PageDirEntry { number, size, address };
        if number <= section_array_size as i32 {
            address += size as u64;
        }
        if number < 0 && pos + 16 <= dec.len() {
            pos += 16; // parent/left/right/x00 gap-tree fields, unused for D1-D2 raw location.
        }
        out.push(entry);
    }
    out
}

#[derive(Debug, Clone)]
struct SectionDescriptor {
    name: String,
    total_size: u64,
    pages: Vec<(i32, u32, u64)>, // (page_number, compressed_size, address_offset_within_section)
    compressed: u32,             // 1 = stored, 2 = LZ-compressed
    max_decomp_size: u32,
}

fn parse_section_info(dec: &[u8]) -> Vec<SectionDescriptor> {
    let u32_at = |o: usize| u32::from_le_bytes(dec[o..o + 4].try_into().unwrap());
    let u64_at = |o: usize| u64::from_le_bytes(dec[o..o + 8].try_into().unwrap());
    let num_desc = u32_at(0);
    let mut pos = 20usize;
    let mut out = Vec::with_capacity(num_desc as usize);
    for _ in 0..num_desc {
        let size = u64_at(pos);
        let num_sections = u32_at(pos + 8);
        let max_decomp_size = u32_at(pos + 12);
        let compressed = u32_at(pos + 20);
        let name_raw = &dec[pos + 32..pos + 32 + 64];
        let end = name_raw.iter().position(|&b| b == 0).unwrap_or(64);
        let name = String::from_utf8_lossy(&name_raw[..end]).into_owned();
        pos += 96;
        let mut pages = Vec::with_capacity(num_sections as usize);
        for _ in 0..num_sections {
            let pnum = i32::from_le_bytes(dec[pos..pos + 4].try_into().unwrap());
            let psize = u32_at(pos + 4);
            let paddr = u64_at(pos + 8);
            pages.push((pnum, psize, paddr));
            pos += 16;
        }
        out.push(SectionDescriptor { name, total_size: size, pages, compressed, max_decomp_size });
    }
    out
}
//#endregion SectionMapAndInfo

//#region Tests
fn fixture_bytes() -> Vec<u8> {
    let path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg";
    fs::read(path).expect("real architectural.dwg fixture must be readable")
}

fn main() {
    let data = fixture_bytes();
    println!("fixture bytes: {}", data.len());

    // ---- D1: version sentinel ----
    assert_eq!(&data[0..6], b"AC1024");
    println!("[ok] version sentinel AC1024");

    // ---- D1: file header decrypt ----
    let enc = &data[0x80..0x80 + 0x6c];
    let dec_hdr = decrypt_r2004_header(enc);
    let hdr = parse_r2004_file_header(&dec_hdr);
    assert_eq!(&hdr.file_id_string[0..11], b"AcFssFcAJMB");
    println!("[ok] file header decrypted, file_ID_string verified: {:?}", String::from_utf8_lossy(&hdr.file_id_string));
    println!("     {:#?}", hdr);

    // ---- D1: section page map (physical page directory) ----
    let map_hdr_addr = hdr.section_map_address as usize + 0x100;
    let map_section_type = u32::from_le_bytes(data[map_hdr_addr..map_hdr_addr + 4].try_into().unwrap());
    assert_eq!(map_section_type, 0x41630e3b, "section page map signature");
    let map_decomp_size = u32::from_le_bytes(data[map_hdr_addr + 4..map_hdr_addr + 8].try_into().unwrap()) as usize;
    let map_comp_size = u32::from_le_bytes(data[map_hdr_addr + 8..map_hdr_addr + 12].try_into().unwrap()) as usize;
    let map_comp = &data[map_hdr_addr + 0x14..map_hdr_addr + 0x14 + map_comp_size];
    let map_dec = decompress_r2004_section(map_comp, map_decomp_size).expect("decompress section page map");
    assert_eq!(map_dec.len(), map_decomp_size);
    let page_dir = parse_page_directory(&map_dec, hdr.section_array_size);
    println!("[ok] section page map decompressed: {} entries", page_dir.len());
    assert_eq!(page_dir.len() as u32, hdr.numgaps + hdr.numsections, "page directory entry count matches header");

    // Independent cross-check: the running "next address" after the last entry must equal
    // last_section_address + 0x100 (a value read from a COMPLETELY different header field,
    // decrypted independently) -- if the LZ decompressor or page-directory parser were wrong,
    // this arithmetic identity would not hold by chance.
    let last = page_dir.last().unwrap();
    let mut running = 0x100u64;
    for e in &page_dir {
        if e.number <= hdr.section_array_size as i32 {
            running += e.size as u64;
        }
    }
    assert_eq!(running, hdr.last_section_address + 0x100, "page directory total size matches independent header field");
    println!("[ok] page directory cross-validated against last_section_address (running={running:#x})");
    let _ = last;

    // Self-consistency: the page-map's own directory entry (number == section_map_id) must
    // report the same address we used to locate it in the first place.
    let map_entry = page_dir.iter().find(|e| e.number == hdr.section_map_id as i32).expect("section_map_id entry present");
    assert_eq!(map_entry.address as usize, map_hdr_addr, "section map self-address matches");
    println!("[ok] section-map self-address cross-validated");

    // ---- D1: section info (named-section directory) ----
    let info_entry = page_dir.iter().find(|e| e.number == hdr.section_info_id).expect("section_info_id entry present");
    let info_addr = info_entry.address as usize;
    let info_section_type = u32::from_le_bytes(data[info_addr..info_addr + 4].try_into().unwrap());
    assert_eq!(info_section_type, 0x4163003b, "section info signature");
    let info_decomp_size = u32::from_le_bytes(data[info_addr + 4..info_addr + 8].try_into().unwrap()) as usize;
    let info_comp_size = u32::from_le_bytes(data[info_addr + 8..info_addr + 12].try_into().unwrap()) as usize;
    let info_comp = &data[info_addr + 0x14..info_addr + 0x14 + info_comp_size];
    let info_dec = decompress_r2004_section(info_comp, info_decomp_size).expect("decompress section info");
    assert_eq!(info_dec.len(), info_decomp_size);
    let descriptors = parse_section_info(&info_dec);
    println!("[ok] section info decompressed: {} descriptors", descriptors.len());

    let expected_names = [
        "AcDb:Header", "AcDb:AuxHeader", "AcDb:Classes", "AcDb:Handles", "AcDb:Template",
        "AcDb:ObjFreeSpace", "AcDb:AcDbObjects", "AcDb:RevHistory", "AcDb:SummaryInfo",
        "AcDb:Preview", "AcDb:AppInfo", "AcDb:AppInfoHistory", "AcDb:FileDepList",
    ];
    for name in expected_names {
        let found = descriptors.iter().any(|d| d.name == name);
        assert!(found, "expected named section {name} not found in real fixture");
        println!("     [found] {name}");
    }

    // ---- D2: decompress every real named section's page content ----
    let by_number: std::collections::HashMap<i32, &PageDirEntry> = page_dir.iter().map(|e| (e.number, e)).collect();
    for desc in &descriptors {
        if desc.name.is_empty() {
            continue; // padding slot (num_desc2 sometimes counts an empty leading descriptor)
        }
        let mut total_decoded = 0usize;
        for &(pnum, _psize, _paddr) in &desc.pages {
            let page = by_number.get(&pnum).unwrap_or_else(|| panic!("page {pnum} for section {} not in directory", desc.name));
            let file_addr = page.address as usize;
            let mut raw32 = [0u8; 32];
            raw32.copy_from_slice(&data[file_addr..file_addr + 32]);
            let ph = decrypt_page_header(&raw32, file_addr);
            assert_eq!(ph.page_type, 0x4163043b, "page_type for section {} page {pnum}", desc.name);
            let comp = &data[file_addr + 32..file_addr + 32 + ph.data_size as usize];
            let content = if desc.compressed == 2 {
                decompress_r2004_section(comp, desc.max_decomp_size.max(ph.page_size) as usize)
                    .unwrap_or_else(|e| panic!("decompress section {} page {pnum}: {e}", desc.name))
            } else {
                comp.to_vec()
            };
            total_decoded += content.len();
        }
        println!("[ok] section {:<20} pages={} total_decoded_bytes={}", desc.name, desc.pages.len(), total_decoded);
    }

    println!("\nALL DWG D1-D2 PRIMITIVES VALIDATED AGAINST REAL FIXTURE");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcg_decrypt_is_symmetric_placeholder_roundtrip() {
        // decrypt_r2004_header is its own inverse (XOR with a deterministic keystream) -- prove
        // that in isolation with synthetic data before trusting it on the real fixture.
        let plain: Vec<u8> = (0..0x6cu8).collect();
        let enc = decrypt_r2004_header(&plain);
        let dec = decrypt_r2004_header(&enc);
        assert_eq!(dec, plain);
    }

    #[test]
    fn lz_round_trip_self_generated_literal_only() {
        // opcode1=0x00 -> zero low bits -> reads a run-length-extension byte; keep it simple:
        // synthesize a stream that's ALL literal (opcode with nonzero low nibble means run-length
        // 0+3..15+3 literal bytes, followed immediately by terminator 0x11).
        let mut comp = vec![0x03u8]; // opcode nibble=3 -> literal length 3+3=6? no: (opcode&0xF)=3, lowbits=3, returns 3+3=6
        comp.extend_from_slice(b"abcdef"); // 6 literal bytes
        comp.push(0x11); // terminator (this becomes the "next opcode" read by copy_bytes' trailing read)
        let out = decompress_r2004_section(&comp, 64).expect("decompress");
        assert_eq!(&out, b"abcdef");
    }

    #[test]
    fn real_fixture_full_pipeline() {
        // The exhaustive version of this test is `main()` above (run via `cargo run`) since it
        // prints diagnostic output; this variant just re-asserts the load-bearing invariants
        // under `cargo test` so CI-style runs catch regressions too.
        let data = fixture_bytes();
        assert_eq!(&data[0..6], b"AC1024");
        let enc = &data[0x80..0x80 + 0x6c];
        let dec_hdr = decrypt_r2004_header(enc);
        let hdr = parse_r2004_file_header(&dec_hdr);
        assert_eq!(&hdr.file_id_string[0..11], b"AcFssFcAJMB");

        let map_hdr_addr = hdr.section_map_address as usize + 0x100;
        let map_decomp_size = u32::from_le_bytes(data[map_hdr_addr + 4..map_hdr_addr + 8].try_into().unwrap()) as usize;
        let map_comp_size = u32::from_le_bytes(data[map_hdr_addr + 8..map_hdr_addr + 12].try_into().unwrap()) as usize;
        let map_comp = &data[map_hdr_addr + 0x14..map_hdr_addr + 0x14 + map_comp_size];
        let map_dec = decompress_r2004_section(map_comp, map_decomp_size).unwrap();
        let page_dir = parse_page_directory(&map_dec, hdr.section_array_size);
        assert_eq!(page_dir.len() as u32, hdr.numgaps + hdr.numsections);

        let info_entry = page_dir.iter().find(|e| e.number == hdr.section_info_id).unwrap();
        let info_addr = info_entry.address as usize;
        let info_decomp_size = u32::from_le_bytes(data[info_addr + 4..info_addr + 8].try_into().unwrap()) as usize;
        let info_comp_size = u32::from_le_bytes(data[info_addr + 8..info_addr + 12].try_into().unwrap()) as usize;
        let info_comp = &data[info_addr + 0x14..info_addr + 0x14 + info_comp_size];
        let info_dec = decompress_r2004_section(info_comp, info_decomp_size).unwrap();
        let descriptors = parse_section_info(&info_dec);
        assert!(descriptors.iter().any(|d| d.name == "AcDb:Header"));
        assert!(descriptors.iter().any(|d| d.name == "AcDb:Classes"));
        assert!(descriptors.iter().any(|d| d.name == "AcDb:Handles"));
    }
}
//#endregion Tests
