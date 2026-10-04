# PNG And Content Clearance Audit

Date: 2026-10-03

Scope: read-only audit of the current PNG canonical-source implementation and the window content-clearance change. No build or test was run by this audit because native fleet work is active.

## P1: `ChangeGamma` Can Emit A Nonconforming Indexed PNG

`ChangeGammaMutation` delegates a missing gAMA insert to `set_ancillary_chunk_controlled` ([change-gamma](../../../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗️change-gamma/🦀️.rs#L24), [IO](../../../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🦀️.rs#L678)). That generic helper inserts every missing ancillary chunk directly before the first IDAT ([IO](../../../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🦀️.rs#L708)).

The admitted `indexed-2bit-duplicate-palette.png` contains `IHDR, PLTE, tRNS, bKGD, IDAT, IEND` and no gAMA; this was directly confirmed from its byte stream. The mutation therefore writes gAMA after PLTE. PNG requires gAMA before PLTE and IDAT, so the editor can publish an invalid image from an admitted source. The local parser does not catch this ordering error ([projection](../../../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🦀️.rs#L923)).

Repair `ChangeGamma` with a gAMA-specific insertion boundary after IHDR and before PLTE/IDAT, retaining all unrelated byte ranges. Refuse zero gAMA too: the mutation accepts `Some(0)` ([change-gamma](../../../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗️change-gamma/🦀️.rs#L11)) and the parser accepts it ([projection](../../../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🦀️.rs#L962)). Add a neutral indexed-without-gAMA fixture that asserts chunk order, exact inverse, and rejection of zero.

## P2: The Admitted PNG Grammar Is Less Strict Than The Checkpoint Claims

`png_layout_bytes` checks framing, CRCs, a first IHDR, presence of IDAT, and projection ([layout](../../../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🦀️.rs#L573)). It does not enforce singleton/placement rules for IHDR, PLTE, IDAT, IEND, or modeled ancillary chunks. For example, `project_png` overwrites a repeated IHDR or PLTE, accepts non-consecutive IDAT, accepts tRNS for alpha colour types, and accepts non-empty IEND ([projection](../../../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🦀️.rs#L923)).

This is why the P1 output survives local reparse. Add one structural-validation pass before projection: exactly one 13-byte IHDR first; one zero-byte IEND last; consecutive IDAT; profile-aware PLTE/tRNS/bKGD cardinality and placement; and singleton rules for modeled chunks. Cover each refusal with a small byte-authored fixture. This preserves the intended exact-source model while making “checked PNG source” mean a standards-conforming source.

## Content Clearance: Observed Gaps Repaired In Current Checkout

The initial implementation disabled a chrome-aware host when any descendant was edgeless and included nested-window overlays in the parent clearance. The current code scopes top overlays to their nearest window body ([deadline helper](../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧱️elements/🚧️WindowContentDeadLine/🟦️.tsx#L23)) and treats only a sole direct edgeless layout child as edgeless ([inset](../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧱️elements/🚧️WindowContentDeadLine/🟦️.tsx#L88)). The neutral fixture now covers both nested-edgeless content and a nested window with lower chrome ([fixture](../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🧫️fixtures/🚧️content-clearance/🔣️.json#L84)); the component law renders both ([test](../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🧪️tests/🧩️component/🟦️.tsx#L169)).

The bottom-anchored expanded utility toolbar is correctly excluded from a top clearance calculation ([Window](../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🟦️.tsx#L406)). The owner reported the corresponding Window suite as green and a live Add-row check as direct; this audit did not independently execute those checks.
