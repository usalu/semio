# Shared Dimensions in the Worked Example

The example now uses these values:

| Form | L | B | H |
| --- | ---: | ---: | ---: |
| Box | 2.5 | 1.5 | 2 |
| Pyramid | 2.5 | 1.5 | 2 |
| Elliptical cylinder | 3.5 | 2.5 | 3 |

Every numeric row contains two equal entries and one different entry. The same first two examples share the complete dimension vector and differ only in Form. Their two directed K3 arrows therefore contain only the Form change. The other four arrows show ±(1,1,1), with the correct categorical direction.

The native cylinder geometry and filled table match these values. The worked-example header is 20 mm high so the taller cylinder fits with padding at the common drawing scale. Its four longer numeric captions use two lines at the existing font size, preventing clipping beneath the node artwork. Capsule tables, complete directed K16, closed 16→1 ring, graph height and twenty answer rows remain unchanged.

## Verification

- Version-ten schema, language-neutral fixtures and independent D3/mathjs checks pass. They verify two-equal/one-different multiplicities on every row, the same shared pair on all three axes, identical complete first/second dimension vectors with different Form, signed vector changes and cycles. Negative cases reject all-equal rows, all-distinct rows, mismatched shared pairs and identical Form.
- Independent native PDF checks pass for the new dimensions, four filled rows, three distinct projected solid geometries, header height including its table rule, four vector captions and six Form descriptions.
- Directed-graph and matrix audits pass: all 240 K16 arcs, twenty selected directions, six K3 arcs, clear arrowheads, the approximately 125.6 mm K16 height and both compact 9-by-9 matrices.
- Native German light, English light and German dark compilation and visual review pass. All three final German pages were rendered and individually inspected; the two-line labels are fully readable. There are no overfull, underfull or missing-character errors in the final German log. All variants retain three pages.
- Bilingual explanatory prose and criteria are complete. Frozen source hashes remained unchanged during final compilation. Repeated header/footer pixels in rendered pages were independently confirmed identical.

## Final Artifact

- output/pdf/Fingeruebung-Parametrisierung.pdf
- Three A4 pages, 134085 bytes
- SHA-256: 579101d898808f7a1c29996900c46f4f40bf3874363b31b598eb455d45aca23c

Authored scripts, neutral inputs and reports remain in this ticket. Temporary compiler outputs and renders are removed after verification. Existing print styling, canonical capsule resources and concurrent repository changes are preserved.
