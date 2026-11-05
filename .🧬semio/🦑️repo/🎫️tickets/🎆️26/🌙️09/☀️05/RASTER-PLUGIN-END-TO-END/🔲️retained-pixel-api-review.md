# Retained Pixel API Static Review

`StepContext::consume_retained` does **not** spend CPU fuel. Its implementation at `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs:248–252` checks retained admission, accumulates the physical receipt and records it in the original return budget. `consume_fuel` at lines 380–381 independently increments actual work and debits semantic/source fuel. The retained admission method's line-257 documentation explicitly states that retained currency is independent of CPU fuel. Every source-copy, image-copy, hash and algorithm driver must separately charge actual work after execution, including zero-copy structural transitions that perform work.

Static inspection of host `owned/🔲️pixel-source`, `owned/🖼️image-copy`, model `schema/🔑️content-hash`, and shared `🔲️pixels/✍️editing` storage extraction found no definite Rust type error. This is not compilation evidence.

Integration constraint: `RasterImageCopyWork::advance` phase 0 rejects replacement images whose dimensions differ from its metadata source and constructs its candidate with the original dimensions. Its replacement path is appropriate for filters/fill and other same-extent edits. A resize/crop/non-square quarter-turn caller must explicitly support changed intrinsic dimensions instead of passing the result to this unchanged API. No production caller currently references this helper, so this is an integration requirement rather than an observed failing route.

Resource handling: source/image reservation phases return observed capacity and attach it to errors when allocation exceeds the admitted quote. Their later chunk copy phases use the reserved vectors. Source preparation keeps descriptor ownership after pixel handoff, and image copying keeps its outer owner after result handoff; both still require supplied-grant closure before Drop. Drivers must not drop either helper merely because `take_pixels`/`take` returned a result. Hash advancement enforces nonzero grants no larger than 32768 bytes and binds to the same source address.

The shared `into_storage` APIs move their vectors and source handles rather than cloning them. Flood storage exposes its source Arc separately; retirement accounting must preserve that shared ownership and must not count a shared source allocation as newly reserved by `into_storage`.

No build or test was executed; the source-admission descriptor-3 failure remains the verification blocker.
