"""🖨️ raster (P2-X): the layer-edit validators named their refusals in the framework's `mutation.*` namespace and the diff
forwarded that string as the report code. The validators now name raster-owned refusals (`raster-layer-not-found`,
`raster.layer.*`, the app faults `edit_fault` raises and the editor declares), and the report is the frozen code the refusal
means (`edit_refusal`: layer gone → TargetMissing, any other rule → Invariant, addressed at the layer)."""
import re

ANY = "🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/"
MUT = ANY + "🧬️schema/🧬️mutations/"
SCHEMA = ANY + "🧬️schema/🦀️.rs"
EDITOR = ANY + "✏️editor/🦀️.rs"
LEAVES = ["🎛️change-layer-adjustment-parameter", "🎨️change-layer-pixels", "🎭️change-layer-mask", "📐️change-layer-transform", "🔒️change-layer-locked"]
OWNED = {
    "mutation.target-missing": "raster-layer-not-found", "mutation.transform-invalid": "raster.layer.transform-invalid",
    "mutation.transform-conflict": "raster.layer.transform-conflict", "mutation.asset-missing": "raster.layer.asset-missing",
    "mutation.invariant": "raster.layer.edit-invalid", "mutation.mask-conflict": "raster.layer.mask-conflict",
    "mutation.adjustment-capacity": "raster.layer.adjustment-capacity", "mutation.adjustment-conflict": "raster.layer.adjustment-conflict",
    "mutation.adjustment-invalid": "raster.layer.adjustment-invalid", "mutation.image-conflict": "raster.layer.image-conflict",
    "mutation.lock-conflict": "raster.layer.lock-conflict",
}


def owned_codes(text: str) -> str:
    """🏷️ Every quoted validator/app-fault literal of the `mutation.*` namespace → its raster-owned code."""
    return re.sub(r'"(mutation\.[a-z-]+)"', lambda match: f'"{OWNED[match.group(1)]}"' if match.group(1) in OWNED else match.group(0), text)


OVERRIDES = {
    (MUT + "🎛️change-layer-adjustment-parameter/🦀️.rs", 29): {"skip": True},
    (MUT + "🎨️change-layer-pixels/🦀️.rs", 33): {"skip": True},
    (MUT + "🎭️change-layer-mask/🦀️.rs", 33): {"skip": True},
    (MUT + "📐️change-layer-transform/🦀️.rs", 18): {"skip": True},
    (MUT + "🔒️change-layer-locked/🦀️.rs", 16): {"skip": True},
}

REFUSAL = "crate::standards::v1::subsets::any::schema::edit_refusal"
EDITS = [
    (MUT + "🎛️change-layer-adjustment-parameter/🦀️.rs",
     'return protocol::MutationOutcome::error(code,"Adjustment parameter cannot be applied to this layer revision.",[self.layer_id.clone()]);',
     f'return {REFUSAL}(code,&self.layer_id);'),
    (MUT + "🎨️change-layer-pixels/🦀️.rs",
     'return protocol::MutationOutcome::error(code, "Pixel content cannot be applied to this image revision.", [self.layer_id.clone()]);',
     f'return {REFUSAL}(code, &self.layer_id);'),
    (MUT + "🎭️change-layer-mask/🦀️.rs",
     'return protocol::MutationOutcome::error(code, "Mask cannot be applied to this layer revision.", [self.layer_id.clone()]);',
     f'return {REFUSAL}(code, &self.layer_id);'),
    (MUT + "📐️change-layer-transform/🦀️.rs",
     'return protocol::MutationOutcome::error(code,"Transform cannot be applied to this layer revision.",[self.layer_id.clone()]);',
     f'return {REFUSAL}(code,&self.layer_id);'),
    (MUT + "🔒️change-layer-locked/🦀️.rs",
     'return protocol::MutationOutcome::error(code,"Layer protection changed before this edit.",[self.layer_id.clone()]);',
     f'return {REFUSAL}(code,&self.layer_id);'),
    (SCHEMA, '        "mutation.target-missing" => semio_framework_plugin::app_fault("mutation.target-missing"),\n', ""),
    (SCHEMA,
     '''pub fn require_layer_edit(''',
     '''/// 🧾️ The mutation report of an edit a mutation validator refused, addressed at its layer: the layer gone → target
/// missing, every other rule → invariant.
pub fn edit_refusal<D: Default>(code: &'static str, layer_id: &str) -> protocol::MutationOutcome<D> {
    if code == "raster-layer-not-found" {
        protocol::MutationOutcome::error(protocol::MutationCode::TargetMissing, [layer_id.to_string()])
    } else {
        protocol::MutationOutcome::error(protocol::MutationCode::Invariant, [layer_id.to_string()])
    }
}

pub fn require_layer_edit('''),
    (EDITOR, '            .fault("mutation.target-missing", LocalizedLabel::native("The layer to change does not exist anymore; choose an existing layer.", "Die zu ändernde Ebene gibt es nicht mehr; wählen Sie eine vorhandene Ebene."))\n', ""),
]

TRANSFORMS = [(MUT + leaf + "/🦀️.rs", owned_codes) for leaf in LEAVES] + [
    (SCHEMA, owned_codes), (EDITOR, owned_codes), (MUT + "📐️change-layer-transform/🧪️tests/🦀️.rs", owned_codes)]
