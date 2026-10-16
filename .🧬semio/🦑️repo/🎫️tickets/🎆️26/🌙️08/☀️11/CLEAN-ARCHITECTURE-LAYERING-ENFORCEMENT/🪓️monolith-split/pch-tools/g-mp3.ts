// 🎵️ mp3: the ID3 metadata IO leaf becomes its own crate; the mutations crate depends on it and the independent-oracle laws move into it as unit tests, so no crate mounts another crate's source.
import {DRY, log, put, replaceOnce, addToSection, lockEnsure, relPosix, ownerRowsAddViaGenerator, ownerRowRemove, deleteFile, read} from "./dd.ts";
const M = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3";
const S = `${M}/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any`;
const IO = `${S}/🚪️io/🏷️metadata`;
const IO_PKG = `${IO}/📦️packages/🦀️rust`;
const SNAP_PKG = `${S}/🧬️schema/📸️snapshot/🏷️metadata/📦️packages/🦀️rust`;
const MUT_PKG = `${S}/🧬️schema/🧬️mutations/📦️packages/🦀️rust`;
const NAME = "semio-s-artifact-stdio-mp3-io-metadata";
const FW = "🧰️framework/🔨️modules";
const dev = [
  ["semio-framework-dsl", `${FW}/🗣️dsl/📦️packages/🦀️rust`],
  ["semio-framework-dsl-record", `${FW}/🗣️dsl/🧬️schema/📦️packages/🦀️rust`],
  ["semio-framework-pack-json", `${FW}/🎒️pack/🔤️json/📦️packages/🦀️rust`],
  ["semio-framework-value", `${FW}/🌱️value/📦️packages/🦀️rust`],
] as const;
const prefix = "crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::";
const src = `${IO}/🦀️.rs`;
const text = read(src);
if (text.split(prefix).length !== 4) throw new Error("expected three metadata model references");
const rewritten = text.replace(`use ${prefix}{Id3Content,Id3Frame,Id3v1Tag,Id3v2Tag,validate_id3_frame,validate_id3v1_tag};`, "use semio_s_artifact_stdio_mp3_metadata::{Id3Content,Id3Frame,Id3v1Tag,Id3v2Tag,validate_id3_frame,validate_id3v1_tag};").replaceAll(`${prefix}id3_content_kind(`, "semio_s_artifact_stdio_mp3_metadata::id3_content_kind(");
if (rewritten.includes(prefix)) throw new Error("model reference left");
put(src, rewritten.replace(/\s*$/u, "\n") + '\n#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod laws;\n');
const laws = `${IO}/🧪️tests/🦀️.rs`;
replaceOnce(laws, `extern crate semio_framework_value_derive as value_derive;
#[path="../../../🧬️schema/📸️snapshot/🏷️metadata/🦀️.rs"]
pub mod admitted;
pub mod standards{pub mod mpeg1_layer3{pub mod subsets{pub mod any{pub mod schema{pub mod snapshot{pub use crate::admitted::*;}}}}}}
#[path="../🦀️.rs"]
pub mod native;
#[cfg(test)]
mod tests{`, `use semio_s_artifact_stdio_mp3_metadata as admitted;
use crate as native;
mod tests{`);
const manifest = `[package]
workspace = "${relPosix(IO_PKG, M)}"
name = "${NAME}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "Component crate of semio-s-artifact-stdio-mp3: the ID3v1 and ID3v2 metadata native IO leaf"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
semio-s-artifact-stdio-mp3-metadata = { path = "${relPosix(IO_PKG, SNAP_PKG)}" }

[dev-dependencies]
${dev.map(([name, dir]) => `${name} = { path = "${relPosix(IO_PKG, dir)}" }`).join("\n")}
id3 = "1.17"
`;
put(`${IO_PKG}/Cargo.toml`, manifest, true);
replaceOnce(`${MUT_PKG}/🦀️.rs`, `                    #[path = "../../../../🚪️io/🏷️metadata/🦀️.rs"]
                    pub mod metadata;`, "                    pub use semio_s_artifact_stdio_mp3_io_metadata as metadata;");
addToSection(`${MUT_PKG}/Cargo.toml`, "[dependencies]", `${NAME} = { path = "${relPosix(MUT_PKG, IO_PKG)}" }`);
lockEnsure(`${M}/Cargo.lock`, NAME, "0.1.0", ["id3", "semio-framework-dsl", "semio-framework-dsl-record", "semio-framework-pack-json", "semio-framework-value", "semio-s-artifact-stdio-mp3-metadata"], ["semio-s-artifact-stdio-mp3-mutations"]);
await ownerRowsAddViaGenerator([{name: NAME, root: IO_PKG}]);
ownerRowRemove("semio-s-artifact-stdio-mp3-metadata-laws");
deleteFile(`${IO}/🧪️tests/📦️packages/🦀️rust/Cargo.toml`);
deleteFile(`${IO}/🧪️tests/📦️packages/🦀️rust/Cargo.lock`);
const script = `${M}/📦️packages/🟦️typescript/📜️script.ts`;
replaceOnce(script, "🚪️io/🏷️metadata/🧪️tests/📦️packages/🦀️rust/Cargo.toml", "🚪️io/🏷️metadata/📦️packages/🦀️rust/Cargo.toml");
log(DRY ? "dry run" : "written");
