# 🖼️ TIFF Oracle Development Reference Closure

The sole Native replay7 compiler refused17 unresolved image/png references in the retained ignored fixture derivation. Actual pre-cut manifest594bytes had only optional tiff and neutral laws. The frozen composition dependency authority declares image0.25, optional true, default-features false, features bmp/tiff/jpeg; png0.18, optional true, original defaults. These original objects remain byte-identical. Native added only the exact development declarations, removing optional as Cargo dev activation differs; normal dependencies and oracles feature are unchanged.

A generic closed language-neutral development-reference owner schema/fixture now binds the actual TIFF private source SHA and original pre-manifest full bytes. The existing owned portable router validates it with our schema validator and independent AJV; independent Bun and existing @iarna TOML parsers agree for both actual captured pre-manifest and actual current manifest. The new law proves pre-manifest has no dev references, current exact dev objects equal original references with only optional removed, unchanged dependency/feature sets, exact original ignored derivation SHA, and retained ignore marker. No format exemption, public thirdparty API, facade, original fixture change or expected SHA relaxation was introduced.

Actual own portable route60pass/2fail/1637assertions across62cases in4.59s; new development closure law passes, all prior61cases retained. The same two historical Shooting/Procedural frozen guards remain RED. This is source/schema/dependency proof; TIFF/full47 runtime remains pending sole Native. Log 🗑️generated/goal-stdio/current-tiff-closed-development-dependencies.log. Real RED authority is actual replay7 compilation and captured pre-source, not a manufactured current-source failure after Native corrected the manifest.

| Input | Bytes | SHA256 |
| --- | --- | --- |
| 🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧬️schema/🧪️development-dependencies/🔣️.json | 1707 | 91a38c369181532ab9df3f2cee03b60ce64aeb6b1cf69be1f6edcd460f644ed3 |
| 🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧫️fixtures/🧪️development-dependencies/🔣️.json | 1253 | fe7c9cd4ccdfc5ebce5f76ed29f603dc13cf378f3fe4a17198d7802ac840177c |
| 🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🧩️composition/🟦️.ts | 26169 | dc57862cf7cadfa87d0a6e17122e198edfaf56df0f6ec4adcb7351bd19086ce3 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🔮️oracles/📦️packages/🦀️rust/Cargo.toml | 736 | ad1a5aaf7ccb9be8b4d70b6b38f0ee023b3e4b5d5192985b65cceec927c9245a |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔮️oracles/🧪️tests/🔬️fixture-derivation/🦀️.rs | 7084 | c05cc66efcf4285a17d1b004ad3faf598cfd8069bbcb5d443dd27d9e24287da7 |

## 🔒️ Full Original Private Derivation Input

Source ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔮️oracles/🧪️tests/🔬️fixture-derivation/🦀️.rs; SHA256 c05cc66efcf4285a17d1b004ad3faf598cfd8069bbcb5d443dd27d9e24287da7.

```rust

use super::*;

fn ifd0_from_image_encoder(rgb: image::RgbImage) -> OracleIfd {
    let mut cursor = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgb8(rgb).write_to(&mut cursor, image::ImageFormat::Tiff).expect("image crate: encode reference TIFF");
    let doc = read_tiff(&cursor.into_inner()).expect("re-parse the reference encoder's own bytes");
    doc.ifds.into_iter().next().expect("reference encoder wrote at least one IFD")
}

/// 🧭️ Walks up from `start` looking for the repo root's own `CLAUDE.md` — robust regardless of
/// how deep the compiling crate's manifest happens to sit (this file is also `#[path]`-included
/// from a throwaway type-checking crate outside the real oracle crate's tree during review).
fn find_repo_root(start: &std::path::Path) -> std::path::PathBuf {
    let mut dir = start.to_path_buf();
    for _ in 0..32 {
        let mut candidate = dir.clone();
        candidate.push("CLAUDE.md");
        if candidate.is_file() {
            return dir;
        }
        if !dir.pop() {
            break;
        }
    }
    panic!("could not find repo root (CLAUDE.md) above {}", start.display());
}

#[test]
#[ignore]
fn derive_real_world_fixture() {
    let repo_root = find_repo_root(std::path::Path::new(env!("CARGO_MANIFEST_DIR")));
    let mut jpeg_path = repo_root.clone();
    jpeg_path.push("🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️assets/🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg");
    let mut png_path = repo_root.clone();
    png_path.push("🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️assets/🏛️rathaus-ahlen-grundriss/🖼️.png");
    let mut out_path = repo_root.clone();
    out_path.push("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🧫️fixtures/🖼️abbau-aufbau-masterarbeit-grundriss.tiff");

    // IFD 0: the real 500 DPI scan, RGB8, via the registered `image` reference encoder.
    let photo = image::open(&jpeg_path).expect("open real JPEG scan").to_rgb8();
    assert_eq!((photo.width(), photo.height()), (2275, 2560), "source scan dimensions moved — re-check the ticket's own numbers");
    let ifd0 = ifd0_from_image_encoder(photo.clone());

    // IFD 1: the real second page — genuine decoded+downsampled pixels of the rathaus PNG,
    // decoded with the registered `png` reference decoder (independent of `image`, which has
    // no PNG feature linked in this crate) then downsampled with `image`'s own generic resize.
    let png_bytes = std::fs::read(&png_path).expect("read real PNG floor plan");
    let mut png_reader = png::Decoder::new(std::io::Cursor::new(&png_bytes)).read_info().expect("png: read_info");
    let mut buf = vec![0u8; png_reader.output_buffer_size().unwrap_or(0)];
    let frame = png_reader.next_frame(&mut buf).expect("png: next_frame");
    let info = png_reader.info();
    let palette = info.palette.clone();
    let trns = info.trns.clone();
    let rgba: Vec<u8> = match frame.color_type {
        png::ColorType::Rgba => buf[..frame.buffer_size()].to_vec(),
        png::ColorType::Rgb => buf[..frame.buffer_size()].chunks_exact(3).flat_map(|p| [p[0], p[1], p[2], 255]).collect(),
        png::ColorType::Grayscale => buf[..frame.buffer_size()].iter().flat_map(|&g| [g, g, g, 255]).collect(),
        png::ColorType::GrayscaleAlpha => buf[..frame.buffer_size()].chunks_exact(2).flat_map(|p| [p[0], p[0], p[0], p[1]]).collect(),
        png::ColorType::Indexed => {
            let table = palette.as_deref().expect("indexed PNG without a palette");
            buf[..frame.buffer_size()]
                .iter()
                .flat_map(|&index| {
                    let base = index as usize * 3;
                    let alpha = trns.as_deref().and_then(|t| t.get(index as usize).copied()).unwrap_or(255);
                    [table[base], table[base + 1], table[base + 2], alpha]
                })
                .collect()
        }
    };
    let full = image::RgbaImage::from_raw(frame.width, frame.height, rgba).expect("rathaus PNG raw buffer matches its own dimensions");
    let small = image::imageops::thumbnail(&full, 16, 16);
    let ifd1 = ifd0_from_image_encoder(image::DynamicImage::ImageRgba8(small).to_rgb8());

    let doc = OracleDoc { little_endian: true, ifds: vec![ifd0, ifd1] };
    let bytes = write_tiff(&doc);

    // Prove the spliced file is genuinely readable back — both by this module's own
    // independent reader AND by the registered `image` decoder (IFD 0 only, its own scope).
    let reparsed = read_tiff(&bytes).expect("re-parse the derived multi-IFD fixture");
    assert_eq!(reparsed.ifds.len(), 2, "derived fixture must carry exactly two real IFDs");
    let (w, h, _) = decode_raster(&reparsed.ifds[0]).expect("decode IFD 0 raster");
    assert_eq!((w, h), (2275, 2560));
    let via_image = image::codecs::tiff::TiffDecoder::new(std::io::Cursor::new(&bytes)).expect("image crate: independently parse derived fixture");
    assert_eq!(image::ImageDecoder::dimensions(&via_image), (2275, 2560));

    std::fs::write(&out_path, &bytes).expect("write committed shared:// fixture");
    eprintln!("wrote {} ({} bytes)", out_path.display(), bytes.len());

    // A SMALL real thumbnail (8x8, genuine decoded+downsampled rathaus pixels — not
    // synthetic) reused inline as `insert-ifd`'s real embedded-IFD content in
    // the feature file's Examples table: printed as hex here rather than committed, since it's
    // small enough to live directly in the feature text (192 bytes = 384 hex chars).
    let tiny = image::imageops::thumbnail(&full, 8, 8);
    let tiny_rgb = image::DynamicImage::ImageRgba8(tiny).to_rgb8();
    let tiny_hex: String = tiny_rgb.as_raw().iter().map(|b| format!("{b:02x}")).collect();
    eprintln!("inline 8x8 real thumbnail hex (insert-ifd pixels): {tiny_hex}");

    // A committed `shared://` binary fixture for `replace-pixels`: the SAME real photo's own
    // pixels, horizontally flipped (still 100% real content, but a genuinely different,
    // provable raster) — full IFD 0 resolution, so it can only reasonably live as a binary
    // fixture, not inline JSON hex.
    let (w, h) = (photo.width(), photo.height());
    let mut flipped_rgba = Vec::with_capacity(w as usize * h as usize * 4);
    for y in 0..h {
        for x in 0..w {
            let px = photo.get_pixel(w - 1 - x, y);
            flipped_rgba.extend_from_slice(&[px[0], px[1], px[2], 255]);
        }
    }
    let mut case_fixture_dir = repo_root.clone();
    case_fixture_dir.push("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🧪️tests/mutate-tiff-6-0/🧫️fixtures");
    std::fs::create_dir_all(&case_fixture_dir).expect("create case fixtures dir");
    let mut flipped_path = case_fixture_dir.clone();
    flipped_path.push("🖼️.rgba");
    std::fs::write(&flipped_path, &flipped_rgba).expect("write shared:// replace-pixels fixture");
    eprintln!("wrote {} ({} bytes, {w}x{h} RGBA8)", flipped_path.display(), flipped_rgba.len());
}

```

## 🪟️ BMP and Note Original Test Reference Closure

Native replay8 terminal compiler RED2m46 refused two original png references in BMP ignored derivation. Only its actual original png0.18 defaults are now added under dev-dependencies; normal image dependency and original provider feature list remain unchanged. Original manifest prefix862bytes SHA61547b687a1d42f633de76d55479d98d9f1e15c749fc10977929afcf03114351; current911bytes SHAd3ba5c47a3846957a1f24f3b992ca3d45f48ac87f73535871a4bbfa30cd2e128. Original ignored derivation3761bytes SHAea6aec380a075d4b83be21cfdedaf3ac6364f56bc69ce48c2bfd8a7017306f60 unchanged.

Bounded independent actual47 package context scan then found Note smoke uses lopdf at38/41, three actual crate roots, with no direct normal/dev declaration. Generic closed test contract was extended before correction; actual source RED59pass/3fail/1659 assertions62cases1.294s explicitly refused Note missing dev-dependencies. Native then added exact frozen lopdf0.44 defaults under dev-dependencies only. Original1036byte manifest6c283560… remains its current1087byte manifest prefix (SHA5cc17a2813ab22f57185ed446a0312ec8e987f67ae68e4b3830452e1b36fc9d8). Original3663byte smoke SHA8ab47ca77e40bb71080cfeb19319769f80a1ec09054bd41784ee9ac96b5f1583 unchanged. Ignore status is declared per actual source: TIFF/BMP true, Note false; no original law becomes ignored.

Current generic schema/fixture binds all3 actual original sources, full original pre-manifests, exact original external definitions, normal feature/dependency preservation, both independent TOML parsers and exact ignore states. Fresh complete owner router60pass/2fail/1664 assertions62cases2.34s; new law GREEN, prior61 required cases retained, same two original frozen guards RED. Log current-tiff-bmp-note-closed-development-dependencies.log; real Note RED retained in current-note-closed-development-dependencies-red.log. No old dependency object or guard SHA changed.

## 🔍️ All Original Native Package Source Reference Audit

Observed 2026-10-02T13:17:00.023Z. Actual47 contract-enumerated native package manifests,177 actual source/package mounts,224 source reads,807 known original frozen external crate root occurrences. Every physically mounted source is included, conservatively covering inline cfg(test) as well as physical test owners; strict Cargo module membership supplies manifest provenance. No missing direct normal/development references or invalid manifests remain after the3 closures. No compiler/runtime is inferred: this lexical/context audit does not replace Rust alias/cfg/type/feature resolution, references outside the26 original external dependency roots, dependency build scripts or native law execution.

| Actual Package | Manifest SHA256 | Mounted Sources | Original External References |
| --- | --- | --- | --- |
| semio-repo-test-host | 002079f76068b77ed7353880cb868b3e9f105183c39241d302b6d6722c9a1bf9 | 7 | 0 |
| semio-s-artifact-stdio-las-test-oracle | 4cf53e4bd89b2e2e485f4ad211bf014f70f09963a34605ed13be270ea8dea221 | 3 | 38 |
| semio-s-artifact-stdio-html-test-oracle | 78b798f3914ea79dccef0e56798313fb2208dbb5008f47e251b72c436b845608 | 2 | 8 |
| semio-s-artifact-stdio-epw-test-oracle | 488b19e4ca2010529b04e2f2ffc11a1e2fa3951143a44d6ce413f5af5200017e | 3 | 3 |
| semio-s-artifact-stdio-zip-test-oracle | 6519fe65b6944e3808b8c28924d47a7701cefdb38bbf3cf2c53917f499b9b28c | 3 | 13 |
| semio-s-artifact-stdio-gif-test-oracle | b40a76ddfd0963dcb9e5d7cacdcd03ca6590f645af6213a7e331752426b028c4 | 3 | 28 |
| semio-s-artifact-stdio-mp4-test-oracle | 227a4a744898052d5bbab782a4e9f662222e71766c7a7a0987afe19a272b16f0 | 2 | 16 |
| semio-s-artifact-stdio-svg-test-oracle | 418dde22a648e07c9300a8b38b5dcb2361843485960826063e3f946f829d3d29 | 4 | 3 |
| semio-s-artifact-stdio-mp3-test-oracle | e10ad47925cf2cad5cdc63db55557459204db4e2c8500560732a6622e471d572 | 3 | 11 |
| semio-s-artifact-stdio-ifc-test-oracle | c251234c94429b261a34664732ce37021c5d0fdcae92c1918b7eea95f04edea1 | 12 | 8 |
| semio-s-artifact-stdio-bcf-test-oracle | 99e9423ad1e397f55a2a86b39bd861a5b7337be8914032e8077a9d4a104dc909 | 2 | 10 |
| semio-s-artifact-stdio-binary-test-oracle | 271b3a1226a68199aa04c1ccf0a16e117444b8d4727704ca2dcd6fdbd358c830 | 3 | 0 |
| semio-s-artifact-stdio-csv-test-oracle | 1f93794f88cee8c54ab155d89f6be5d67dc8ea83a54286a2c2ce0adedbe6cacc | 3 | 2 |
| semio-s-artifact-stdio-step-test-oracle | cf856073df713dd6e48290ff578c91e5ed0a8e43234cf4245aa3ebd3dfc586d7 | 17 | 7 |
| semio-s-artifact-stdio-tsv-test-oracle | c19925bb7794ad0d09c0a48d00e9bd600c75ae885e3e68bed5ec6f55e03499cb | 3 | 5 |
| semio-s-artifact-stdio-xlsx-test-oracle | 134c18b2307f7aeb9828ef7f585f392c2b5fec58d507ec9cb72849fbd5caaca2 | 7 | 9 |
| semio-s-artifact-stdio-pdf-test-oracle | 1fce67624be54b24dc0be98fc270513b1911b4f82b23675372a3f107d06a900b | 21 | 6 |
| semio-s-artifact-stdio-docx-test-oracle | 4c60243c06b1e151808ffb4ab0f82f7e92e49942d2569741a52ea3fa32e09537 | 7 | 15 |
| semio-s-artifact-stdio-md-test-oracle | a6b023ce2aa103b1a02cd56a90f3e74ca75116be41534f4d097592b85d8dc718 | 2 | 2 |
| semio-s-artifact-stdio-xml-test-oracle | 4a0ab476a31f3fb127cef77cacdae145e7cc60068f7edef94b163c4d465f81b5 | 3 | 5 |
| semio-s-artifact-stdio-png-test-oracle | 5ff0bff7a6ee83a3b365fbc1da44ddeed839c2d73cc873f675436dccc42899ee | 2 | 30 |
| semio-s-artifact-stdio-jpg-test-oracle | 8ea3e20bcadedfbd2c90cec742c4ac4f4c0df8f5239a20227766f6e97c3a2b76 | 4 | 10 |
| semio-s-artifact-stdio-avi-test-oracle | 1d1a112dbc2c6c02325298b26a77da649ecd5b8b37381b94f9c305f7d9d2a0ae | 2 | 1 |
| semio-s-artifact-stdio-pptx-test-oracle | 6cb1cca395b44d4be05e3492d54a586a2b95ffa261a8461bee1a41879bd565c9 | 7 | 11 |
| semio-s-artifact-stdio-wav-test-oracle | 2bbc2aacee34df11e423307b22f084fc5744ab51d56a6b48807056529d65128a | 2 | 1 |
| semio-s-artifact-stdio-txt-test-oracle | cbdab921db1f0998e190fdb5b6475480ff0055b12b9899545cb46b17fa9f7ffb | 3 | 3 |
| semio-s-artifact-stdio-stl-test-oracle | 6091b9c2b0d88a23147b7558895d57f1d248f62994057a13a1544c5588e7ed48 | 3 | 1 |
| semio-s-artifact-stdio-dwg-test-oracle | b93205b665875a4a9ee31ca7b9c12b56ce3142504b03d51c8ff54a62ff5a0a0c | 4 | 0 |
| semio-s-artifact-stdio-dxf-test-oracle | 35521e83239b86e43ea44fb51f9f23b3c36e59d6423294079c36ab6db4d31fbb | 3 | 4 |
| semio-s-artifact-stdio-tiff-test-oracle | ad1a5aaf7ccb9be8b4d70b6b38f0ee023b3e4b5d5192985b65cceec927c9245a | 5 | 20 |
| semio-s-artifact-stdio-deflate-test-oracle | 36f4ab4d4b31ea174532a2710e07b709f6dcde5a311929b760d6a4d9bd089302 | 2 | 1 |
| semio-s-artifact-stdio-obj-test-oracle | af5d445db95f4faf93437046da99e73b39943c8185b451e6da4d2e4bef38b9a1 | 3 | 0 |
| semio-s-artifact-stdio-gltf-test-oracle | 294a4459c32b0dafb57fa029172dfb39e920d1d60c4b38bfbf6dd717a9ad4b61 | 3 | 403 |
| semio-s-artifact-stdio-ply-test-oracle | 2b312a5c4e9f39b755dbd59218fdd1bffeee385798941c912535ce5e0b6a24fe | 2 | 12 |
| semio-s-artifact-stdio-json-test-oracle | 31e136ac9a6f4900fa06f9733730f5eb54a90fa2b1cd3264a0621e08675469bb | 3 | 37 |
| semio-s-artifact-stdio-semio-test-oracle | 66a14941878dda9127c1e8818dda29a12860661beea5a9e6ef6bb032fdb1bcbd | 2 | 9 |
| semio-s-artifact-stdio-bmp-test-oracle | d3ba5c47a3846957a1f24f3b992ca3d45f48ac87f73535871a4bbfa30cd2e128 | 3 | 9 |
| semio-s-artifact-note-note-test-oracle | 5cc17a2813ab22f57185ed446a0312ec8e987f67ae68e4b3830452e1b36fc9d8 | 3 | 4 |
| semio-s-plugin-stdio-archive-test-oracle | 18f450bc8fca175e6eab0f8b2964ad2a28b68826187f0c314b0d5db6d77819a0 | 1 | 8 |
| semio-s-plugin-stdio-document-test-oracle | 4e9935abb0d3b5676c6dc10508a54fabb3c0122ce117dc62197e0a8f30470032 | 1 | 20 |
| semio-s-plugin-stdio-tabular-test-oracle | c80223ffaf88ba6d8bcaf36e83a397006cfb413f7b5d681c2420f701554f8f97 | 1 | 2 |
| semio-s-plugin-stdio-markup-test-oracle | aa23d636c991eba4140b9634b39482fb782b9a3d43d1d3ef02d7092221b00c04 | 1 | 3 |
| semio-s-plugin-stdio-audio-test-oracle | 4465e04135acedffb125f7685b50c3ac9a8734613753858c94efb4d5efc8b99c | 2 | 0 |
| semio-s-plugin-stdio-raster-test-oracle | f541a8735ae51279b3ba703eb6bc80fec7ff304fb5146d8ef93a6cf44b95c164 | 2 | 21 |
| semio-s-plugin-stdio-mesh-test-oracle | e73b89748215b061dfb2810c4ce5aa38cc16f3a2440a8f1fca51ca4096b84770 | 1 | 8 |
| semio-s-plugin-stdio-part21-test-oracle | 5ed2f5645b42afb370fa45ab46f4ca92a91ff3dd1289ca84bfa5fa51003c0196 | 1 | 0 |
| semio-hub-stdio-test-oracle | 08b8890ed5b46b82eaed3a5e139202fd716b2014b7feb18d0adfe75af3f38fd6 | 1 | 0 |

## 🔒️ Full Original BMP Private Test Input

```rust

/// 🧭️ Walks up from `start` looking for the repo root's own `CLAUDE.md`.
fn find_repo_root(start: &std::path::Path) -> std::path::PathBuf {
    let mut dir = start.to_path_buf();
    for _ in 0..32 {
        let mut candidate = dir.clone();
        candidate.push("CLAUDE.md");
        if candidate.is_file() {
            return dir;
        }
        if !dir.pop() {
            break;
        }
    }
    panic!("could not find repo root (CLAUDE.md) above {}", start.display());
}

#[test]
#[ignore]
fn derive_real_world_fixture() {
    let repo_root = find_repo_root(std::path::Path::new(env!("CARGO_MANIFEST_DIR")));
    let mut png_path = repo_root.clone();
    png_path.push("🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️assets/🏛️rathaus-ahlen-grundriss/🖼️.png");
    let mut out_path = repo_root.clone();
    out_path.push("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🏛️rathaus-ahlen-grundriss/🖼️.bmp");

    let png_bytes = std::fs::read(&png_path).expect("read the real PNG floor plan");
    let mut reader = png::Decoder::new(std::io::Cursor::new(&png_bytes)).read_info().expect("png: read_info");
    let mut buffer = vec![0u8; reader.output_buffer_size().unwrap_or(0)];
    let frame = reader.next_frame(&mut buffer).expect("png: next_frame");
    assert_eq!(frame.color_type, png::ColorType::Indexed, "the source is an indexed PNG; a resolved one would lose the palette this fixture exists for");
    let table = reader.info().palette.clone().expect("indexed PNG carries a PLTE");
    let mut palette: Vec<[u8; 3]> = table.chunks_exact(3).map(|entry| [entry[0], entry[1], entry[2]]).collect();
    let indices = buffer[..frame.buffer_size()].to_vec();
    assert_eq!(palette.len(), 233, "the real PLTE moved — re-check the padding arithmetic below");

    let mut referenced = [false; 256];
    for index in &indices {
        referenced[*index as usize] = true;
    }
    assert!(referenced[..palette.len()].iter().all(|used| *used), "every real palette entry is referenced; the padding below is what creates the slack");

    let spare: Vec<[u8; 3]> = (0u16..=255).map(|value| [value as u8, 0, (255 - value) as u8]).filter(|candidate| !palette.contains(candidate)).take(7).collect();
    assert_eq!(spare.len(), 7, "the deterministic ramp must yield seven colours the real table does not already hold");
    palette.extend(spare);
    assert_eq!(palette.len(), 240);
    assert!(indices.iter().all(|index| (*index as usize) < 233), "no pixel may reference a spare entry");

    let mut bytes = Vec::new();
    image::codecs::bmp::BmpEncoder::new(&mut bytes).encode_with_palette(&indices, frame.width, frame.height, image::ExtendedColorType::L8, Some(&palette)).expect("image crate: write the indexed BMP");

    let reparsed = super::oracles::decode(&bytes).expect("re-parse the derived fixture with the independent reader");
    assert_eq!((reparsed.width, reparsed.height), (frame.width, frame.height));
    match &reparsed.content {
        super::oracles::Content::Indexed { indices: back, palette: table } => {
            assert_eq!(table.len(), 240, "the derived fixture must declare its full 240-entry table");
            assert_eq!(back, &indices, "the index buffer must survive the round trip through the reference encoder");
        }
        super::oracles::Content::Direct { .. } => panic!("the derived fixture decoded as direct-colour; the palette was lost"),
    }

    std::fs::write(&out_path, &bytes).expect("write the committed shared:// fixture");
    eprintln!("wrote {} ({} bytes, {}x{}, 240-entry table)", out_path.display(), bytes.len(), frame.width, frame.height);
}

```

## 🔒️ Full Original Note Smoke Test Input

```rust

use super::{project_note_dxf, project_note_pdf, project_note_svg};

/// 🖊️ A minimal DXF R12 `ENTITIES` section holding exactly the shape `NoteIntoDxf::serialize`
/// emits for one Ink block's `points.windows(2)` pair: one `LINE` on layer `"0"`.
const DXF_ONE_LINE: &str = "0\nSECTION\n2\nENTITIES\n0\nLINE\n8\n0\n10\n0.0\n20\n0.0\n30\n0.0\n11\n10.0\n21\n20.0\n31\n0.0\n0\nENDSEC\n0\nEOF\n";

#[test]
fn project_note_dxf_reads_the_line_entity_note_would_have_written() {
    let projected = project_note_dxf(DXF_ONE_LINE.as_bytes()).expect("dxf crate parses a minimal ENTITIES section");
    let entities = projected.array("entities");
    assert_eq!(entities.len(), 1, "expected exactly the one LINE entity");
    assert_eq!(entities[0].str("entityKind"), "line");
}

/// 🎨️ The exact `<g transform="matrix(a,b,c,d,e,f)"><path d="…"/></g>` shape
/// `svg_element_from_draw_node` (the semio/drawing→svg composer note's real bridge dispatches
/// through) writes for one block: a translate-by-(5,10) group wrapping an ink path.
const SVG_ONE_GROUP: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"100\" height=\"100\"><g id=\"layer-0\"><g transform=\"matrix(1,0,0,1,5,10)\"><path d=\"M0,0 L1,1\"/></g></g></svg>";

#[test]
fn project_note_svg_decomposes_the_group_transform_and_reaches_the_path() {
    let projected = project_note_svg(SVG_ONE_GROUP.as_bytes()).expect("quick-xml parses well-formed SVG");
    let root = projected.get("root").expect("root present");
    assert_eq!(root.str("name"), "svg");
    let outer_layer_group = &root.array("children")[0];
    let block_group = &outer_layer_group.array("children")[0];
    let transform = block_group.get("transform").expect("transform decomposed, not left as a raw string");
    // 🔺 `MarkupTransformOp::Matrix{a,b,c,d,e,f}` projected positionally — e/f are the translation
    // this subject's `note_block_transform` would have written for `x: 5.0, y: 10.0`.
    assert!(format!("{transform:?}").contains('5'), "expected the translate-x component 5 somewhere in {transform:?}");
    let path = &block_group.array("children")[0];
    assert_eq!(path.str("name"), "path");
}

#[test]
fn project_note_pdf_reads_the_text_lopdf_itself_wrote() {
    use lopdf::{Document, Object, Stream, dictionary};
    let mut document = Document::with_version("1.4");
    let pages_id = document.new_object_id();
    let content = lopdf::content::Content { operations: vec![lopdf::content::Operation::new("Tj", vec![Object::string_literal("hello from note")])] };
    let content_id = document.add_object(Stream::new(dictionary! {}, content.encode().expect("encode content stream")));
    let page_id = document.add_object(dictionary! { "Type" => "Page", "Parent" => pages_id, "Contents" => content_id, "MediaBox" => vec![0.into(), 0.into(), 200.into(), 100.into()] });
    let pages = dictionary! { "Type" => "Pages", "Kids" => vec![page_id.into()], "Count" => 1 };
    document.objects.insert(pages_id, Object::Dictionary(pages));
    let catalog_id = document.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages_id });
    document.trailer.set("Root", catalog_id);
    let mut bytes = Vec::new();
    document.save_to(&mut bytes).expect("lopdf saves the document it just built");

    let projected = project_note_pdf(&bytes).expect("lopdf reads back its own document");
    let pages_json = projected.array("pages");
    assert_eq!(pages_json.len(), 1);
    let text = pages_json[0].array("text");
    assert!(text.iter().any(|value| matches!(value, semio_repo_test_host::Json::String(s) if s == "hello from note")), "expected the Tj text operand to surface, got {text:?}");
}

```

## 📋️ Fresh Bounded Input Handoff

Observed 2026-10-02T13:19:14.157Z. Authored current roster696 inputs; retains all previous694 and adds only2 closed development contract inputs. Ordered SHA256 ee01170870cfad8ca43e44eeaef7d51d2fce1bb34cde9e96addea7d0577d13fd. Changed previous input observations are recorded below; original677/686/694 epochs remain historical authority. Normal package Cargo.lock observations are captured separately as generated dependency resolution, never authored source or frozen corpus authority; subsequent normal Cargo replay may legitimately update them. TIFF actual native2pass/1originalignored receipt is Native evidence in its main report, not a result executed by this worker. BMP/Note and complete47/deletion execution remain pending.

```json
{
  "at": "2026-10-02T13:19:14.157Z",
  "digest": "ee01170870cfad8ca43e44eeaef7d51d2fce1bb34cde9e96addea7d0577d13fd",
  "changed": [
    {
      "path": ".vscode/🧩️launch.seed.jsonc",
      "previousSha256": "9c25452a92a80a96dc32b25b3830077fd33de823aa87d41a1431eb951938df19",
      "currentSha256": "6166a9deaf97a42f67ff7d081af04d49051f58fb23dc3da371f9cc416b65441b"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🔮️oracles/📦️packages/🦀️rust/Cargo.toml",
      "previousSha256": "97e4f19d20342f401b348dc30902686e82877b9e8a9c4bb341ac3d2ecaa6a4bc",
      "currentSha256": "ad1a5aaf7ccb9be8b4d70b6b38f0ee023b3e4b5d5192985b65cceec927c9245a"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🔮️oracles/📦️packages/🦀️rust/Cargo.toml",
      "previousSha256": "61547b687a1d42f633de76d55479d98d9f1e15c749fc10977929afcf03114351",
      "currentSha256": "d3ba5c47a3846957a1f24f3b992ca3d45f48ac87f73535871a4bbfa30cd2e128"
    },
    {
      "path": "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🔮️oracles/📦️packages/🦀️rust/Cargo.toml",
      "previousSha256": "6c283560027b0f5676a452b9b0899ae62741354a3ceb1662d217b018a87266a0",
      "currentSha256": "5cc17a2813ab22f57185ed446a0312ec8e987f67ae68e4b3830452e1b36fc9d8"
    },
    {
      "path": "🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🧩️composition/🟦️.ts",
      "previousSha256": "2f7ebeb14a113b7ccf999c98b6fdace827da4fd52e19845e93cb91611c9af42b",
      "currentSha256": "006ebbddc348762348d912ee6dd19d7a7e0cee85d33583291aa8981939015384"
    }
  ],
  "generatedDependencyAuthority": [
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "31e32479a545b1d968895cb5c1540983159acce645bc4154afce7066375a0ca9",
      "bytes": 164,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "7516f7a6f77191e3224431b53adf0c6a5ad057c06a432b73c70b43e358b9af44",
      "bytes": 10096,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "b867a1b49f55db214afed02b9e319a64d505089f67685273a0da6a782edd4b7b",
      "bytes": 8128,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "069b2c4c14079fc7031ec8388694f840af035d10e20f8f481fcc78a9a64b4f6b",
      "bytes": 2726,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "e65c88a2c7e38304ca7a186e832d387e095ac453648adf95afea19f7b41dc083",
      "bytes": 4452,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "5c61b04be00eded3eb6e266dea2d278cdf9523fceafc5889549e229e58a4824b",
      "bytes": 7982,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "ab79368b5873e6d0477f4b5846ba85787a66a3f509c4b2e9d658b3f39f20c6f9",
      "bytes": 5346,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "ed5fc351785e0a10328367b087d298e8f21a9068cb423944a10df5f521d22c12",
      "bytes": 897,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "ffd729f1774d00ef07bec5a46a971aa8fa31ffbd0c1714ff1c1849b38f9d7f47",
      "bytes": 2411,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "04951760290d3123db7de0e85775528dbca7c7095f48858c8fcebc7486983ad4",
      "bytes": 9851,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "0fa13f35690e77e41bf063cc073e8a6501c3e92cb7e85d54110b7ac5906a6da5",
      "bytes": 4690,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "0f3a0d81d12be00a9f5f800e93b419de3ffde264364400c5e1fffbf0e4a76887",
      "bytes": 290,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "1970f5ab325409d36aa8e9defb380230f3329e9e0da53a5d5c0b61d14c04a36e",
      "bytes": 2726,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "9b06d2c73ad07308dff67eadf2814ec7d070ec3892d1e209b92536be010eef2e",
      "bytes": 9852,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "92c06c5845d895373b5111c5ff495969c7ace8f4989179713a00e36e3751f228",
      "bytes": 2726,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "7617b1672612b8d3b98407869c13e620f5af4dcdd63330e2d5982d3c2301cba2",
      "bytes": 30539,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "0baeeffc19fb5c00ecb0330c104d902d0ea6517715486b16fd530b273e158a7e",
      "bytes": 27631,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "51cce0de79a1c805a479532b3763ae51cbdbf9099e5cfee1ab2d38623b635d75",
      "bytes": 27644,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "9b256ff59f4ee0859ac6849ff38d69361ae44748034f81610edf7532b5bae690",
      "bytes": 23368,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "dce170b192cbfc7227f4a1e5d4a9f62f955756b1b57d382d4d73940870e2348a",
      "bytes": 897,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "b6c944bbdcf2222f897e75d05da77f2352da783e126e3fbb66a21fcb4ca461d2",
      "bytes": 2729,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "119087cce3fa77b8808b64604949e4a05a0ba5e826f575122ea8b53e9b085674",
      "bytes": 6413,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "67e74e1da063de1f4d8192d15670305dc319882706a4ce46289e45de6c9527af",
      "bytes": 484,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "9f0dbe93a353bef6198cb49ca3aa17503f8ebdb7c33b82222bca6606dec34d07",
      "bytes": 27644,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "c779aa05a7ca87cd38d30599f4338235e5d439b5475ff31ad24e04cd4037bffb",
      "bytes": 650,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "6c5663b15bea5569947fd412406f2569d96ba72d8d137e51257c5d553c9e2c75",
      "bytes": 2969,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "2a4d57d2787d8ef0cffb1fc364b5a56c7664dc400872f88437075add03c77b27",
      "bytes": 1372,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "6b58286e671090764ce397be28e5df99c98e697476822cbdf1f4decd042d01e2",
      "bytes": 287,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "bd4cce2360bdd55c8338024c432264fc00a2b2cde0de84634e84d0c043d8a8e1",
      "bytes": 15874,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "5c71fb54ba973d1aa9208f67ca70566f9f14bf0c271f0efd111a8cfc7b157325",
      "bytes": 7380,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "9e4b90ae3197289488c8e51ce6031819731de9ff042f8585148a19252f513f7a",
      "bytes": 5110,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "6ccdf6170cfbe139539b1dc4d396fc150dd77afd0bcc6e93fc926e802262733a",
      "bytes": 287,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "cf2cf1df26bedfa9a084d790db6295cf85c883175b99fb98e3440b74dc7c711c",
      "bytes": 486,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "528b8510b01141c7da8bcd016c8e71519c1b5f383a3dba4835c3d67ac048245e",
      "bytes": 10151,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "5865e5a2a25dd01e4776bdc618e8f7c228b1ef6f57f3816f6d19026bc388220e",
      "bytes": 486,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "516d1cf6f82b27402f34d198a5da43e1f2e1896b405c0460252c3b6b9cee9aec",
      "bytes": 487,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "sha256": "ee0b91fdbfc7c566bf212680f48715b65d3f5633203c0777c49216a4453dd068",
      "bytes": 7984,
      "authority": "Normal Cargo-generated dependency resolution; separate from authored source input"
    },
    {
      "path": "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "present": false,
      "authority": "No generated lock authority inferred"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🎒️archive/📦️packages/🦀️rust/Cargo.lock",
      "present": false,
      "authority": "No generated lock authority inferred"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📃️document/📦️packages/🦀️rust/Cargo.lock",
      "present": false,
      "authority": "No generated lock authority inferred"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📊️tabular/📦️packages/🦀️rust/Cargo.lock",
      "present": false,
      "authority": "No generated lock authority inferred"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📰markup/📦️packages/🦀️rust/Cargo.lock",
      "present": false,
      "authority": "No generated lock authority inferred"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🔊️audio/📦️packages/🦀️rust/Cargo.lock",
      "present": false,
      "authority": "No generated lock authority inferred"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖼️raster/📦️packages/🦀️rust/Cargo.lock",
      "present": false,
      "authority": "No generated lock authority inferred"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🧊️mesh/📦️packages/🦀️rust/Cargo.lock",
      "present": false,
      "authority": "No generated lock authority inferred"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🔤️part21/📦️packages/🦀️rust/Cargo.lock",
      "present": false,
      "authority": "No generated lock authority inferred"
    },
    {
      "path": "🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/📦️packages/🦀️rust/Cargo.lock",
      "present": false,
      "authority": "No generated lock authority inferred"
    }
  ]
}
```
