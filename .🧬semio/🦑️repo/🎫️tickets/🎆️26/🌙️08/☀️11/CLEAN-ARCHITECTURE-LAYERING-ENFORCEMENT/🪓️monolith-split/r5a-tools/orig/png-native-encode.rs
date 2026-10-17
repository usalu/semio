//! ✍️ Native encoding of the PNG codec: the Latin-1 helpers, the controlled chunk collector, the text chunk encoder and the controlled image encoder and projection. See [`super`].

use super::*;

//#region ExactAuthority
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9

pub(super) fn latin1_text(bytes: &[u8]) -> String { bytes.iter().map(|byte| char::from(*byte)).collect() }

pub(super) fn latin1_bytes(value: &str) -> Vec<u8> { value.chars().map(|character| character as u8).collect() }

struct PngEncodedChunks {pub(super) chunks:Vec<([u8;4],Vec<u8>)>,file_size:usize,maximum_file:usize}

impl PngEncodedChunks {
    pub(super) fn push(&mut self,kind:[u8;4],data:&[u8],control:&mut NativeEncodeControl<'_>)->Result<(),ValueError> {
        if data.len()>u32::MAX as usize {return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"PNG chunk exceeds native length"));}
        self.file_size=self.file_size.checked_add(12).and_then(|n|n.checked_add(data.len())).filter(|n|*n<=self.maximum_file).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"PNG output exceeds file limit"))?;
        self.chunks.push((kind,control.copy_bytes(data)?));Ok(())
    }
}

fn png_latin1_encode(text:&str,control:&mut NativeEncodeControl<'_>)->Result<Vec<u8>,ValueError> {let mut output=control.allocate_vec(text.len())?;control.begin_stage(text.len())?;for character in text.chars() {output.push(u8::try_from(character as u32).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"PNG text exceeds Latin-1"))?);control.advance(character.len_utf8())?;}Ok(output)}

fn png_text_native(text:&PngTextChunk,maximum_file:usize,control:&mut NativeEncodeControl<'_>)->Result<([u8;4],Vec<u8>),ValueError> {
    let keyword=png_latin1_encode(&text.keyword,control)?;
    let value=if text.kind==PngTextKind::IText {control.copy_bytes(text.value.as_bytes())?} else {png_latin1_encode(&text.value,control)?};
    let compressed=if text.compressed {semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::binary::snapshot::compress_zlib(&value,maximum_file,control)?} else {value};
    let extra=match text.kind {PngTextKind::Text=>1,PngTextKind::ZText=>2,PngTextKind::IText=>5+text.language_tag.len()+text.translated_keyword.len()};
    let size=keyword.len().checked_add(extra).and_then(|n|n.checked_add(compressed.len())).filter(|n|*n<=maximum_file).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"PNG text exceeds file limit"))?;
    let mut data=control.allocate_vec(size)?;data.extend_from_slice(&keyword);data.push(0);
    let kind=match text.kind {PngTextKind::Text=>*b"tEXt",PngTextKind::ZText=>{data.push(0);*b"zTXt"},PngTextKind::IText=>{data.extend_from_slice(&[u8::from(text.compressed),0]);data.extend_from_slice(text.language_tag.as_bytes());data.push(0);data.extend_from_slice(text.translated_keyword.as_bytes());data.push(0);*b"iTXt"}};
    control.begin_stage(compressed.len())?;for piece in compressed.chunks(256) {data.extend_from_slice(piece);control.advance(piece.len())?;}Ok((kind,data))
}

pub(super) fn encode_png_image(image:&PngImage)->Result<Vec<u8>,String> {let mut callback=|_|true;let mut control=NativeEncodeControl::new(1024*1024*1024,&mut callback);encode_png_image_controlled(image,512*1024*1024,&mut control).map_err(ValueError::into_message)}

pub(super) fn validate_png_image_controlled(image:&PngImage,checkpoint:&mut dyn FnMut(bool,usize)->Result<(),ValueError>)->Result<(),ValueError> {
    let invalid=|message:&str|ValueError::new(ValueRefusalKind::InvalidValue,message);
    image.validate_header().map_err(|message|invalid(&message))?;
    let maximum=if image.bit_depth==16 {u16::MAX}else{(1u16<<image.bit_depth)-1};
    checkpoint(true,image.samples.len())?;
    for sample in &image.samples {if *sample>maximum||image.color_type==PngColorType::Palette&&usize::from(*sample)>=image.palette.as_ref().map_or(0,Vec::len) {return Err(invalid("png: native sample precision or palette identity differs from the owned profile"));}checkpoint(false,1)?;}
    for text in &image.text_chunks {
        checkpoint(true,1)?;
        if text.keyword.is_empty()||text.kind==PngTextKind::Text&&text.compressed||text.kind==PngTextKind::ZText&&!text.compressed {return Err(invalid("png: text metadata differs from its native text profile"));}
        checkpoint(false,1)?;
        let mut length=0;checkpoint(true,text.keyword.len())?;
        for character in text.keyword.chars() {length+=1;if length>79||character=='\0'||character as u32>255 {return Err(invalid("png: text metadata differs from its native text profile"));}checkpoint(false,character.len_utf8())?;}
        if text.kind!=PngTextKind::IText&&(!text.language_tag.is_empty()||!text.translated_keyword.is_empty()) {return Err(invalid("png: Latin-1 text metadata contains an international-only field"));}
        checkpoint(true,text.value.len())?;for character in text.value.chars() {if text.kind!=PngTextKind::IText&&character as u32>255 {return Err(invalid("png: Latin-1 text metadata contains an international-only field"));}checkpoint(false,character.len_utf8())?;}
        checkpoint(true,text.language_tag.len())?;for byte in text.language_tag.bytes() {if !byte.is_ascii() {return Err(invalid("png: international text language tag must be ASCII"));}checkpoint(false,1)?;}
    }
    let mut after=false;checkpoint(true,image.ancillary_chunks.len())?;
    for chunk in &image.ancillary_chunks {if !chunk.kind.iter().all(u8::is_ascii_alphabetic)||!chunk.kind[0].is_ascii_lowercase()||!chunk.kind[2].is_ascii_uppercase()||[*b"tRNS",*b"gAMA",*b"cHRM",*b"sRGB",*b"pHYs",*b"tIME",*b"bKGD",*b"tEXt",*b"zTXt",*b"iTXt"].contains(&chunk.kind)||after&&!chunk.after_raster {return Err(invalid("png: opaque ancillary metadata overlaps a typed field or has invalid placement"));}after|=chunk.after_raster;checkpoint(false,1)?;}
    Ok(())
}

pub(super) fn encode_png_image_controlled(image:&PngImage,maximum_file:usize,control:&mut NativeEncodeControl<'_>)->Result<Vec<u8>,ValueError> {
    validate_png_image_controlled(image,&mut |start,amount|if start {control.begin_stage(amount)}else{control.advance(amount)})?;
    let color=image.color_type.to_u8();let spp=image.color_type.samples_per_pixel();
    let ihdr=Ihdr {width:image.width,height:image.height,bit_depth:image.bit_depth,color_type:color,interlace:u8::from(image.interlace)};
    let bpp=bpp_bytes(&ihdr);let mut passes=Vec::with_capacity(7);
    if image.interlace {for(pass,&(sx,sy,dx,dy))in ADAM7.iter().enumerate() {let(width,height)=adam7_pass_dims(image.width,image.height,pass);passes.push((sx,sy,dx,dy,width,height));}}else{passes.push((0,0,1,1,image.width,image.height));}
    let raw_size=passes.iter().try_fold(0usize,|total,&(_,_,_,_,width,height)| {if width==0||height==0 {return Ok(total);}total.checked_add((packed_row_bytes(width,color,image.bit_depth)+1).checked_mul(height as usize).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"PNG raw extent overflow"))?).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"PNG raw extent overflow"))})?;
    let mut raw=control.allocate_vec(raw_size)?;
    for(sx,sy,dx,dy,width,height)in passes {
        if width==0||height==0 {continue;}let row_bytes=packed_row_bytes(width,color,image.bit_depth);let mut previous=None;
        for y in 0..height {
            let mut row=control.allocate_vec(row_bytes)?;row.resize(row_bytes,0);control.begin_stage(width as usize*spp)?;
            for x in 0..width {let source=(((sy+y*dy)as usize*image.width as usize)+(sx+x*dx)as usize)*spp;for channel in 0..spp {write_native_sample(&mut row,x as usize*spp+channel,image.bit_depth,image.samples[source+channel]);control.step()?;}}
            let(filter,filtered)=choose_filter_controlled(&row,previous.as_deref(),bpp,control)?;raw.push(filter);control.begin_stage(filtered.len())?;for piece in filtered.chunks(256) {raw.extend_from_slice(piece);control.advance(piece.len())?;}previous=Some(row);
        }
    }
    let compressed=semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::binary::snapshot::compress_zlib(&raw,maximum_file,control)?;
    let count=14usize.checked_add(image.text_chunks.len()).and_then(|n|n.checked_add(image.ancillary_chunks.len())).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"PNG chunk count overflow"))?;
    let mut output=PngEncodedChunks {chunks:control.allocate_vec(count)?,file_size:8,maximum_file};
    let mut header=[0;13];header[..4].copy_from_slice(&image.width.to_be_bytes());header[4..8].copy_from_slice(&image.height.to_be_bytes());header[8..].copy_from_slice(&[image.bit_depth,color,0,0,u8::from(image.interlace)]);output.push(*b"IHDR",&header,control)?;
    if let Some(gamma)=image.gamma {output.push(*b"gAMA",&gamma.to_be_bytes(),control)?;}
    if let Some(c)=image.chromaticities {let mut data=[0;32];for(index,field)in[c.white_x,c.white_y,c.red_x,c.red_y,c.green_x,c.green_y,c.blue_x,c.blue_y].into_iter().enumerate() {data[index*4..index*4+4].copy_from_slice(&field.to_be_bytes());}output.push(*b"cHRM",&data,control)?;}
    if let Some(intent)=image.srgb {output.push(*b"sRGB",&[intent.to_u8()],control)?;}
    if let Some(palette)=&image.palette {let mut data=control.allocate_vec(palette.len()*3)?;control.begin_stage(palette.len())?;for entry in palette {data.extend_from_slice(&[entry.r,entry.g,entry.b]);control.step()?;}output.push(*b"PLTE",&data,control)?;}
    if let Some(transparency)=&image.transparency {match transparency {PngTransparency::Indexed {alpha}=>output.push(*b"tRNS",alpha,control)?,PngTransparency::Grayscale {gray}=>output.push(*b"tRNS",&gray.to_be_bytes(),control)?,PngTransparency::Rgb {r,g,b}=>{let mut data=[0;6];data[..2].copy_from_slice(&r.to_be_bytes());data[2..4].copy_from_slice(&g.to_be_bytes());data[4..].copy_from_slice(&b.to_be_bytes());output.push(*b"tRNS",&data,control)?;}}}
    if let Some(background)=&image.background {let data=encode_bkgd(background);control.charge(data.capacity())?;output.push(*b"bKGD",&data,control)?;}
    if let Some(p)=image.physical_dims {let mut data=[0;9];data[..4].copy_from_slice(&p.ppu_x.to_be_bytes());data[4..8].copy_from_slice(&p.ppu_y.to_be_bytes());data[8]=u8::from(p.unit_is_meter);output.push(*b"pHYs",&data,control)?;}
    if let Some(t)=image.timestamp {let mut data=[0;7];data[..2].copy_from_slice(&t.year.to_be_bytes());data[2..].copy_from_slice(&[t.month,t.day,t.hour,t.minute,t.second]);output.push(*b"tIME",&data,control)?;}
    for text in &image.text_chunks {let(kind,data)=png_text_native(text,maximum_file,control)?;output.push(kind,&data,control)?;}
    for chunk in image.ancillary_chunks.iter().filter(|chunk|!chunk.after_raster) {output.push(chunk.kind,&chunk.data,control)?;}
    output.push(*b"IDAT",&compressed,control)?;
    for chunk in image.ancillary_chunks.iter().filter(|chunk|chunk.after_raster) {output.push(chunk.kind,&chunk.data,control)?;}
    output.push(*b"IEND",&[],control)?;
    let mut bytes=control.allocate_vec(output.file_size)?;bytes.extend_from_slice(&PNG_SIGNATURE);
    for(kind,data)in output.chunks {bytes.extend_from_slice(&(data.len()as u32).to_be_bytes());bytes.extend_from_slice(&kind);let mut crc=0xffff_ffffu32;for byte in kind {crc^=u32::from(byte);for _ in 0..8 {crc=if crc&1!=0 {(crc>>1)^0xedb88320}else{crc>>1};}}control.begin_stage(data.len())?;for piece in data.chunks(256) {bytes.extend_from_slice(piece);for byte in piece {crc^=u32::from(*byte);for _ in 0..8 {crc=if crc&1!=0 {(crc>>1)^0xedb88320}else{crc>>1};}}control.advance(piece.len())?;}bytes.extend_from_slice(&(!crc).to_be_bytes());}
    Ok(bytes)
}

pub fn project_png(bytes: &[u8]) -> Result<PngProjection, String> {
    let image = decode_png_image(bytes)?;
    let pixels = crate::schema::operations::png_rgba8_preview(&image)?;
    let mut chunk_order = vec![PngChunkMarker::Ihdr];
    if image.gamma.is_some() { chunk_order.push(PngChunkMarker::Gama); }
    if image.chromaticities.is_some() { chunk_order.push(PngChunkMarker::Chrm); }
    if image.srgb.is_some() { chunk_order.push(PngChunkMarker::Srgb); }
    if image.palette.is_some() { chunk_order.push(PngChunkMarker::Plte); }
    if image.transparency.is_some() { chunk_order.push(PngChunkMarker::Trns); }
    if image.background.is_some() { chunk_order.push(PngChunkMarker::Bkgd); }
    if image.physical_dims.is_some() { chunk_order.push(PngChunkMarker::Phys); }
    if image.timestamp.is_some() { chunk_order.push(PngChunkMarker::Time); }
    chunk_order.extend((0..image.text_chunks.len()).map(|index|PngChunkMarker::Text { index }));
    let mut unknown_chunks=Vec::new();
    for chunk in image.ancillary_chunks.iter().filter(|chunk|!chunk.after_raster) { chunk_order.push(PngChunkMarker::Unknown { index:unknown_chunks.len() }); unknown_chunks.push(PngChunk { kind:chunk.kind,data:chunk.data.clone() }); }
    chunk_order.push(PngChunkMarker::Idat);
    for chunk in image.ancillary_chunks.iter().filter(|chunk|chunk.after_raster) { chunk_order.push(PngChunkMarker::Unknown { index:unknown_chunks.len() }); unknown_chunks.push(PngChunk { kind:chunk.kind,data:chunk.data.clone() }); }
    chunk_order.push(PngChunkMarker::Iend);
    Ok(PngProjection { width:image.width,height:image.height,bit_depth:image.bit_depth,color_type:image.color_type,interlace:image.interlace,plte:image.palette,trns:image.transparency,gama:image.gamma,chrm:image.chromaticities,srgb:image.srgb,phys:image.physical_dims,time:image.timestamp,bkgd:image.background,text_chunks:image.text_chunks,pixels,chunk_order,unknown_chunks })
}
//#endregion Codec
