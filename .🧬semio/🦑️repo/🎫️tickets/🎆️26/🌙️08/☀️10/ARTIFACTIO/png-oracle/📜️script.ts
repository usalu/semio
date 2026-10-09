import {readFileSync,writeFileSync} from "node:fs";
import {resolve,join} from "node:path";
const ticket=resolve(import.meta.dir,".."),owner="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🔮️oracles/🦀️.rs";
const source=`//! 🔮️ Independent PNG oracle with exact owned samples and metadata.
use semio_repo_test_host::Json;
#[cfg(feature="oracles")]
#[path="🌱️owned-document/🦀️.rs"]
mod owned;
#[cfg(feature="oracles")]
pub struct PngSourceOracleProjection {pub width:u32,pub height:u32,pub bit_depth:u8,pub color_type:u8,pub interlaced:bool,pub rgba:Vec<u8>}
/// 🔬️ Reads the original profile and an independent display projection.
#[cfg(feature="oracles")]
pub fn project_png_source(input:&[u8])->Result<PngSourceOracleProjection,String>{
 let metadata=png::Decoder::new(std::io::Cursor::new(input)).read_info().map_err(|error|error.to_string())?;
 let info=metadata.info();let(width,height,bit_depth,color_type,interlaced)=(info.width,info.height,info.bit_depth as u8,info.color_type as u8,info.interlaced);
 let mut decoder=png::Decoder::new(std::io::Cursor::new(input));decoder.set_transformations(png::Transformations::ALPHA|png::Transformations::STRIP_16);
 let mut reader=decoder.read_info().map_err(|error|error.to_string())?;let mut buffer=vec![0;reader.output_buffer_size().ok_or("PNG oracle display extent")?];let frame=reader.next_frame(&mut buffer).map_err(|error|error.to_string())?;
 let bytes=&buffer[..frame.buffer_size()];let rgba=match frame.color_type{png::ColorType::Rgba=>bytes.to_vec(),png::ColorType::Rgb=>bytes.chunks_exact(3).flat_map(|value|[value[0],value[1],value[2],255]).collect(),png::ColorType::Grayscale=>bytes.iter().flat_map(|value|[*value,*value,*value,255]).collect(),png::ColorType::GrayscaleAlpha=>bytes.chunks_exact(2).flat_map(|value|[value[0],value[0],value[0],value[1]]).collect(),_=>return Err("PNG independent display transform retained indexed samples".into())};
 Ok(PngSourceOracleProjection{width,height,bit_depth,color_type,interlaced,rgba})
}
/// 🌱️ Admits the exact independent owned document.
#[cfg(feature="oracles")]
pub fn project_png_owned(input:&[u8])->Result<Json,String>{owned::reading(input)}
/// 🔑️ Computes the structural revision from independent owned fields.
#[cfg(feature="oracles")]
pub fn owned_png_revision(snapshot:&Json)->Result<String,String>{owned::revision(snapshot)}
#[cfg(feature="oracles")]
pub fn oracle_apply_mutation(input:&[u8],spec:&Json)->Result<Vec<u8>,String>{owned::apply(input,spec)}
#[cfg(feature="oracles")]
pub fn oracle_undo_mutation(original:&[u8],_spec:&Json,mutated:&[u8])->Result<Vec<u8>,String>{owned::reading(mutated)?;owned::encode(&owned::reading(original)?)}
/// 👁️ Compares the native profile, all metadata, and exact sample digest.
#[cfg(feature="oracles")]
pub fn project_png_mutation(bytes:&[u8])->Result<Json,String>{owned::project(bytes)}
#[cfg(feature="oracles")]
pub fn oracle_identity_round_trip(input:&[u8])->Result<Vec<u8>,String>{owned::encode(&owned::reading(input)?)}
#[cfg(not(feature="oracles"))]
pub fn oracle_apply_mutation(_input:&[u8],_spec:&Json)->Result<Vec<u8>,String>{Err("the oracles feature is disabled".into())}
#[cfg(not(feature="oracles"))]
pub fn oracle_undo_mutation(_original:&[u8],_spec:&Json,_mutated:&[u8])->Result<Vec<u8>,String>{Err("the oracles feature is disabled".into())}
#[cfg(not(feature="oracles"))]
pub fn project_png_mutation(_bytes:&[u8])->Result<Json,String>{Err("the oracles feature is disabled".into())}
#[cfg(not(feature="oracles"))]
pub fn oracle_identity_round_trip(_input:&[u8])->Result<Vec<u8>,String>{Err("the oracles feature is disabled".into())}
`;
writeFileSync(owner,source);
writeFileSync(join(ticket,"png-owned-oracle-closure.md"),"# PNG Owned Oracle Closure\n\nReplaced the old native-byte mutation payload oracle with independent exact native sample admission and materialization. The existing third-party png reader and writer remain test-only; flate2 builds independent Adam7 streams when the third-party encoder cannot select interlace. Structural revisions derive from the typed image. Comparisons include source precision, profile, exact sample digest and all admitted metadata. Native carrier byte identity is replaced by owned semantic identity. Verification is pending.\n\n- `"+owner+"`\n- `"+owner.replace("🦀️.rs","🌱️owned-document/🦀️.rs")+"`\n");
process.stdout.write("[DEBUG] replaced PNG oracle owner with independent owned document\n");
