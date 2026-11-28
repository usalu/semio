import sys,os;sys.path.insert(0,os.path.dirname(__file__))
from lib import edit
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
edit(L+"🗂️workspaces/🦀️cargo/🛠️preparation/🟦️.ts",[
(" if(!(error instanceof CargoPreparationManifestChangedV1)||attempt>=cargoPreparationReplanPolicyV1.maximumAttempts)throw error;\n check();await control.publish("," if(!(error instanceof CargoPreparationManifestChangedV1)||attempt>=cargoPreparationReplanPolicyV1.maximumAttempts)throw error;\n console.error(`[cargo-preparation] replan ${attempt}: ${error.message}`);check();await control.publish("),
])
edit(L+"🏗️build/🕸️graph/🦀️rust/📥️intent/🟦️.ts",[
('if(!value.fallback)throw Error("Selected Rust bounded fallback was not declared");','if(!value.fallback)throw Error("Selected Rust bounded fallback was not declared for unsupported units: "+unsupported.map(row=>row.unit+" ("+row.adapters.join("+")+")").join(", ")+" / Begrenzter Rust-Ersatzweg für nicht unterstützte Einheiten nicht deklariert");'),
])
