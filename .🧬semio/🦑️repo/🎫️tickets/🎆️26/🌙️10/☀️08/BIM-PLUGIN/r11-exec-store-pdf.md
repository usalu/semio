# r11-store-pdf: semio-s-artifact-stdio-pdf against the 677 retirement API

## Result
Gate `cargo check --keep-going -p semio-s-artifact-stdio-pdf --lib` (via the bim model manifest): 94 errors at start, then 8, then 0. The final run (`🗑️generated/r11-store-pdf/c4.txt`) ends with `Finished dev profile`. Only `--lib` was checked, not `--tests`.

## Changes (all in `📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema`)
Added `semio_framework_value::RetireOwned` as the first derive on:
- `📸️snapshot/🦀️.rs`: `PdfObject`, `PdfSnapshot`
- `🔺️diff/🦀️.rs`: `PdfIndexedItem<T>`, `PdfIndexedDiff<T>`, `PdfPathSegment`, `PdfPageBox`
- `🏅️conformance-support/🦀️.rs`: `ObjectPlacement`

The other leaves and mutations in the pdf crate (1.4 and 1.7 subsets) already carried the derive.

No `InteractiveJob` or `ArtifactApp` hooks live in this crate, so none were needed.

## Framework dependency
The last 8 errors (`[String; 2]` and `[Vec<u8>; 2]`) needed `impl<T: RetireOwned, const N: usize> RetireOwned for [T; N]` in the value crate. r11-store landed it at `🌱️value/♻️retirement/🦀️.rs:422`. No pdf code changes were needed for it.

## Leftovers
None in the pdf lib. The `--tests` target of the pdf crate was not checked.
