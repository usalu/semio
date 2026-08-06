import fs from "fs";
import path from "path";

const OS = fs.readFileSync("/tmp/os-path.txt","utf8").trim();
const FW = fs.readFileSync("/tmp/fw-path.txt","utf8").trim();
const TICKET = fs.readFileSync("/tmp/os-ticket-path.txt","utf8").trim();
const KERNEL_PKG = path.join(OS, "📦️packages/🦀️rust");
const log = [];
const note = (m) => { console.log(m); log.push(m); };

// 1) PresencePeer: move to spr wire if still in framework-core ui kernel
const kernelUi = path.join(FW, "🔨️modules/🧩core/🧩️ui/🧠️kernel/🦀️component.rs");
const wireComp = path.join(OS, "🔨️modules/📡️spr/📡️wire/🦀️component.rs");
let ui = fs.readFileSync(kernelUi, "utf8");
const presenceRe = /\/\/#region 🔖️Presence[\s\S]*?\/\/#endregion 🔖️Presence\n/;
if (ui.includes("pub struct PresencePeer") && presenceRe.test(ui)) {
  let presence = ui.match(presenceRe)[0].replace(/protocol_core::/g, "crate::os_spr::core::");
  let wire = fs.readFileSync(wireComp, "utf8");
  if (!wire.includes("pub struct PresencePeer")) {
    fs.writeFileSync(wireComp, wire + "\n\n" + presence);
    note("appended PresencePeer to spr wire");
  }
  ui = ui.replace(presenceRe, `//#region 🔖️Presence
pub use semio_framework_os_kernel::{PresencePeer, PresencePoint, PresenceViewport, decode_presence_peer, encode_presence_peer};
//#endregion 🔖️Presence
`);
  fs.writeFileSync(kernelUi, ui);
  note("framework-core PresencePeer -> reexport from kernel");
} else {
  note("PresencePeer already relocated or missing struct");
}

// 2) spr facade reexports
const sprComp = path.join(OS, "🔨️modules/📡️spr/🦀️component.rs");
let spr = fs.readFileSync(sprComp, "utf8");
if (!spr.includes("write_str,")) {
  spr = spr.replace(
    "pub use crate::os_spr::core::{ActorId, ConflictRule, DocumentId, DocumentVersion, HybridLogicalTimestamp, MergeStrategyKind, OperationId, PayloadHash, SchemaId, SchemaVersion, StateClass, UndoPolicy};",
    "pub use crate::os_spr::core::{ActorId, ConflictRule, DocumentId, DocumentVersion, HybridLogicalTimestamp, MergeStrategyKind, OperationId, PayloadHash, SchemaId, SchemaVersion, StateClass, UndoPolicy, read_f64, read_str, read_varint_u64, write_f64, write_str, write_varint_u64};",
  );
  note("added wire codec reexports to spr");
}
if (!spr.includes("PresencePeer")) {
  spr = spr.replace(
    "pub use crate::os_spr::wire::{decode_client_frame, decode_server_frame, encode_client_frame, encode_server_frame, AckStage, ApplyOutcome, Bootstrap, ClientFrame, Lane, ServerFrame};",
    "pub use crate::os_spr::wire::{decode_client_frame, decode_server_frame, encode_client_frame, encode_server_frame, decode_presence_peer, encode_presence_peer, AckStage, ApplyOutcome, Bootstrap, ClientFrame, Lane, PresencePeer, PresencePoint, PresenceViewport, ServerFrame};",
  );
  note("added PresencePeer reexports to spr");
}
fs.writeFileSync(sprComp, spr);

// 3) store: remove config_spec + semio_format path
const storeComp = path.join(OS, "🔨️modules/🏪️store/🦀️component.rs");
let store = fs.readFileSync(storeComp, "utf8");
if (store.includes("semio_framework_core::ConfigSpec")) {
  const before = store;
  store = store.replace(
    /pub fn config_spec_from_record_spec[\s\S]*?Ok\(\(\)\)\n\}\n\/\/#endregion 🔖️Config/,
    `// config_spec_* removed — ConfigSpec is UI (framework-core); avoids kernel↔core cycle\n//#endregion 🔖️Config`,
  );
  if (store === before) {
    // try alternate end
    store = store.replace(
      /pub fn config_spec_from_record_spec[\s\S]*?\/\/#endregion 🔖️Config/,
      `// config_spec_* removed — ConfigSpec is UI (framework-core); avoids kernel↔core cycle\n//#endregion 🔖️Config`,
    );
  }
  note(store.includes("semio_framework_core::ConfigSpec") ? "WARN config_spec still present" : "removed config_spec from store");
}
if (store.includes("pub use semio_format;")) {
  store = store.replace("pub use semio_format;", "pub use crate::os_semio as semio_format;");
  note("store semio_format -> crate::os_semio");
}
fs.writeFileSync(storeComp, store);

// 4) sync retarget
const syncComp = path.join(OS, "🔨️modules/🏪️store/🔄️sync/🦀️component.rs");
if (fs.existsSync(syncComp)) {
  let sync = fs.readFileSync(syncComp, "utf8");
  sync = sync
    .replace("use semio_framework_core::PresencePeer;", "use crate::os_spr::PresencePeer;")
    .replace("use semio_framework_core::DocumentId;", "use crate::os_spr::core::DocumentId;")
    .replace(/semio_framework_core::encode_presence_peer/g, "crate::os_spr::encode_presence_peer")
    .replace(/semio_framework_core::decode_presence_peer/g, "crate::os_spr::decode_presence_peer")
    .replace(/semio_framework_core::(ActorId|OperationId)/g, "crate::os_spr::core::$1");
  fs.writeFileSync(syncComp, sync);
  note("store sync retargeted");
}

// 5) kernel glue: wire semio, rename lib->glue, strip framework-core + semio_format deps
let libPath = path.join(KERNEL_PKG, "📦️lib.rs");
let gluePath = path.join(KERNEL_PKG, "📦️glue.rs");
let glueSrc = fs.existsSync(gluePath) ? gluePath : libPath;
let glue = fs.readFileSync(glueSrc, "utf8");
if (!glue.includes("os_semio")) {
  if (!glue.includes("extern crate self as semio_format")) {
    glue = glue.replace("extern crate self as vcs;", "extern crate self as vcs;\nextern crate self as semio_format;");
  }
  glue = glue.replace(
    "pub use crate::os_vcs::*;",
    `#[path = "../../🔨️modules/🧬️semio/🦀️component.rs"]
pub mod os_semio;

pub use crate::os_vcs::*;
pub use crate::os_semio::*;`,
  );
  note("wired os_semio");
}
fs.writeFileSync(gluePath, glue);
if (fs.existsSync(libPath) && libPath !== gluePath) fs.unlinkSync(libPath);

let cargo = fs.readFileSync(path.join(KERNEL_PKG, "Cargo.toml"), "utf8");
cargo = cargo.replace(/path = "📦️lib\.rs"/, 'path = "📦️glue.rs"');
cargo = cargo.replace(/^semio-framework-core = \{[^}]+\}\n/m, "");
cargo = cargo.replace(/^semio_format = \{[^}]+\}\n/m, "");
if (!cargo.includes('name = "semio"')) {
  cargo += `\n[[bin]]\nname = "semio"\npath = "../../🔨️modules/🧬️semio/📦️bin.rs"\n`;
}
fs.writeFileSync(path.join(KERNEL_PKG, "Cargo.toml"), cargo);
note("kernel Cargo.toml updated");

// 6) framework-core retarget to kernel
const coreCargoPath = path.join(FW, "📦️packages/🦀️rust/Cargo.toml");
let coreCargo = fs.readFileSync(coreCargoPath, "utf8");
const kernelDepBlock = `# 🎯️ OS kernel owns spr/dsl types — aliases avoid dual trees / cycle with old implementations.
semio-framework-os-kernel = { path = "../../🛍️products/💻️os/📦️packages/🦀️rust", package = "semio-framework-os-kernel" }
protocol_core = { path = "../../🛍️products/💻️os/📦️packages/🦀️rust", package = "semio-framework-os-kernel" }
protocol = { path = "../../🛍️products/💻️os/📦️packages/🦀️rust", package = "semio-framework-os-kernel" }
dsl = { path = "../../🛍️products/💻️os/📦️packages/🦀️rust", package = "semio-framework-os-kernel" }
`;
if (coreCargo.includes("protocol_core = { path =") && coreCargo.includes("⚡️implementations")) {
  coreCargo = coreCargo.replace(
    /# 🎞️ CW3[\s\S]*?dsl = \{[^}]+\}\n/,
    kernelDepBlock,
  );
  // fallback if comment pattern differs
  if (coreCargo.includes("⚡️implementations") && coreCargo.includes("protocol_core")) {
    coreCargo = coreCargo.replace(/^protocol_core = \{[^}]+\}\n/m, "");
    coreCargo = coreCargo.replace(/^protocol = \{[^}]+\}\n/m, "");
    coreCargo = coreCargo.replace(/^dsl = \{[^}]+\}\n/m, "");
    // remove orphan comments about CW3/W6
    coreCargo = coreCargo.replace(/^# 🎞️ CW3[\s\S]*?names \(see `kernel`\n# module\) so every existing internal\/external reference keeps resolving unchanged\.\n/m, "");
    coreCargo = coreCargo.replace(/^# 🎯️ W6[\s\S]*?just protocol_core\)\.\n/m, "");
    if (!coreCargo.includes('semio-framework-os-kernel = { path = "../../🛍️products')) {
      coreCargo = coreCargo.replace(
        /^semio-framework-hash = \{[^}]+\}\n/m,
        (m) => m + kernelDepBlock,
      );
    }
  }
  fs.writeFileSync(coreCargoPath, coreCargo);
  note("framework-core retargeted to kernel");
} else if (!coreCargo.includes('package = "semio-framework-os-kernel"')) {
  // force rewrite of the three deps
  coreCargo = coreCargo.replace(/^protocol_core = \{[^}]+\}\n/m, 'protocol_core = { path = "../../🛍️products/💻️os/📦️packages/🦀️rust", package = "semio-framework-os-kernel" }\n');
  coreCargo = coreCargo.replace(/^protocol = \{[^}]+\}\n/m, 'protocol = { path = "../../🛍️products/💻️os/📦️packages/🦀️rust", package = "semio-framework-os-kernel" }\n');
  coreCargo = coreCargo.replace(/^dsl = \{[^}]+\}\n/m, 'dsl = { path = "../../🛍️products/💻️os/📦️packages/🦀️rust", package = "semio-framework-os-kernel" }\n');
  if (!coreCargo.includes("semio-framework-os-kernel =")) {
    coreCargo = coreCargo.replace(
      /^semio-framework-hash = \{[^}]+\}\n/m,
      (m) => m + `semio-framework-os-kernel = { path = "../../🛍️products/💻️os/📦️packages/🦀️rust", package = "semio-framework-os-kernel" }\n`,
    );
  }
  fs.writeFileSync(coreCargoPath, coreCargo);
  note("framework-core deps rewritten");
} else {
  note("framework-core already on kernel");
}

fs.writeFileSync(path.join(TICKET, "🧪cycle-break-log.txt"), log.join("\n")+"\n");
console.log("cycle-break done");
