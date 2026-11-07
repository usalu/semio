#!/bin/bash
# run ONLY after C/🗑️generated/1105b/sync-done exists
cd /Users/ueli/Documents/semio || exit 1
J="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job"
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1105b-tools/runtime/declapp-split"
mkdir -p "$J/🎬️original-command/📦️packages/🦀️rust" && cp "$C/Cargo.toml" "$J/🎬️original-command/📦️packages/🦀️rust/Cargo.toml"
python3 - <<'PY'
J="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job"
def sub(p,o,n):
    s=open(p,encoding='utf-8').read(); assert s.count(o)==1,(p,o); open(p,'w',encoding='utf-8').write(s.replace(o,n))
sub("Cargo.toml",'    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🧱️ownership/📦️packages/🦀️rust",\n','    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🧱️ownership/📦️packages/🦀️rust",\n    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🎬️original-command/📦️packages/🦀️rust",\n')
sub("🧰️framework/Cargo.toml",'    "🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🧱️ownership/📦️packages/🦀️rust",\n','    "🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🧱️ownership/📦️packages/🦀️rust",\n    "🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🎬️original-command/📦️packages/🦀️rust",\n')
A="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/📦️packages/🦀️rust/Cargo.toml"
sub(A,'semio-plugin-original-job-ownership = {','semio-plugin-decl-original-command = { path = "../../../👷️job/🎬️original-command/📦️packages/🦀️rust" }\nsemio-plugin-original-job-ownership = {')
L=J+"/🦀️.rs"
sub(L,'#[path="🎮️command/🎯️action/🦀️.rs"]\npub mod original_action_command;\npub use original_action_command::{ArtifactOriginalCommandProducer,ArtifactOriginalCommandProducerFactory};\n','pub use semio_plugin_decl_original_command::{ArtifactOriginalCommandProducer,ArtifactOriginalCommandProducerFactory};\n')
sub(L,'#[path="⌨️typing/🔗️fold/🦀️.rs"]\npub mod original_typing_fold;\npub use original_typing_fold::{OriginalTypingFoldPolicy,ArtifactOriginalTypingFoldFactory,ArtifactOriginalTypingFoldProducer};\n','pub use semio_plugin_decl_original_command::{OriginalTypingFoldPolicy,ArtifactOriginalTypingFoldFactory,ArtifactOriginalTypingFoldProducer};\n')
PY
rm -f "$J/🎮️command/🎯️action/🦀️.rs" "$J/⌨️typing/🔗️fold/🦀️.rs"; rmdir "$J/🎮️command/🎯️action" "$J/⌨️typing/🔗️fold" "$J/⌨️typing" 2>/dev/null
echo switched
