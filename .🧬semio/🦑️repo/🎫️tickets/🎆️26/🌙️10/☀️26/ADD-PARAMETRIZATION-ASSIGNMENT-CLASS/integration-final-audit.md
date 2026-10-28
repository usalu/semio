# Final Assignment Ownership Audit

The canonical assignment leaf contains only the eleven authored source/test inputs below. No PDF or PNG exists there. The root output/pdf directory is absent. The old print assignment leaf retains only its reusable TypeScript metadata API and schema; its resources directory and exercise sources are absent. Print catalog has no parametrization entries. Standard architecture course targets build-parametrization-en and build-parametrization-de exist; the Print project has no parametrization producer targets.

## Guard Review

The final document intent normalizes resource names with NFC and lowercase and rejects duplicates/prefix collisions. The renderer now obtains intrinsic metadata, limits projected output pixels to the policy maximum before rasterizing, attaches an AbortSignal listener that destroys the Sharp image, removes the listener and destroys the image in finally. All current plan hashes were previously independently checked against sixteen canonical SVG bytes. These are read-only source inspections, not a claim of successful full PDF build.

## Authored Path Inventory

- 🎓️teaching/🏛️architecture/📐️parametric/📝️assignment/de.content.tex
- 🎓️teaching/🏛️architecture/📐️parametric/📝️assignment/de.tex
- 🎓️teaching/🏛️architecture/📐️parametric/📝️assignment/en.content.tex
- 🎓️teaching/🏛️architecture/📐️parametric/📝️assignment/en.tex
- 🎓️teaching/🏛️architecture/📐️parametric/📝️assignment/geometry.tex
- 🎓️teaching/🏛️architecture/📐️parametric/📝️assignment/icons.json
- 🎓️teaching/🏛️architecture/📐️parametric/📝️assignment/🧪️tests/🔮️svg-provenance.py
- 🎓️teaching/🏛️architecture/📐️parametric/📝️assignment/🧪️tests/🟦️.ts
- 🎓️teaching/🏛️architecture/📐️parametric/📝️assignment/🧪️tests/🥒️.feature
- 🎓️teaching/🏛️architecture/📐️parametric/📝️assignment/🧪️tests/🧫️cases.json
- 🎓️teaching/🏛️architecture/📐️parametric/📝️assignment/🧪️tests/🧬️cases.schema.json

## Exact Existing Index Hazard

All nineteen generated PDF additions remain staged while absent on disk (AD). The first column A means their bytes remain in the existing index; second-column D means deletion is only in working tree. No Git mutation was authorized or performed.

```text
AD output/pdf/Assignment-Table-Silhouette.pdf
AD output/pdf/Fingeruebung-Parametrisierung-Ohne-Fliesstext.pdf
AD output/pdf/Fingeruebung-Parametrisierung.pdf
AD 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-capsule-J.pdf
AD 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-capsule-L.pdf
AD 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-capsule-backslash.pdf
AD 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-capsule-p.pdf
AD 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-capsule-q.pdf
AD 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-capsule-s.pdf
AD 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-capsule-slash.pdf
AD 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-capsule-z.pdf
AD 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-ellipsoid-capsule-J.pdf
AD 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-ellipsoid-capsule-L.pdf
AD 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-ellipsoid-capsule-backslash.pdf
AD 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-ellipsoid-capsule-p.pdf
AD 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-ellipsoid-capsule-q.pdf
AD 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-ellipsoid-capsule-s.pdf
AD 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-ellipsoid-capsule-slash.pdf
AD 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/resources/metabolism-ellipsoid-capsule-z.pdf
```
