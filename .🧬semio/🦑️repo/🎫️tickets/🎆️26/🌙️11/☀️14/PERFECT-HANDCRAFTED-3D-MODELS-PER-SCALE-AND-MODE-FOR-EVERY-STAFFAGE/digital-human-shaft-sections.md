# Human shaft sections across drawing scales

The 28 human catalog studies were compared by paired femoral shafts. Eight previously published adults, the two schoolchildren and the carried toddler already had individual shaft-section studies. Seventeen additional adults retained coarse anatomical errors. The stroller’s child already had its own 16 mm femoral shafts at each scale. The new study evaluates the adult occupants and leaves the child’s anatomy and all equipment geometry in place.

The fixture cuts posed triangles with planes normal to each left and right femur, tibia, fibula, humerus, radius and ulna at three distances along the limb. It compares both orthogonal diameters and each centre, and checks the physical extent of four joint markers on each side, in two poses at four scales. The runner’s right residual lower leg uses its 180 mm segment and cuts at 34%, 48% and 61%, within both residual bones. Its socket, adapter and carbon running blade remain independent mechanical members.

| Subject | Source SHA-256 prefix | Largest original shaft diameter change (mm) | Original shaft-centre shift (mm) | Fitted diameter change (mm) | Fitted centre shift (mm) |
|---|---|---:|---:|---:|---:|
| cyclist | `5d2073b5b9f2e777` | 53.474 | 0.234 | 0.247 | 0.123 |
| wheelchair-user | `11cc33f9896fdc35` | 55.953 | 20.171 | 0.948 | 0.152 |
| parent-with-stroller | `27ab476f6e1e8fce` | 54.114 | 3.895 | 0.540 | 0.172 |
| courier-with-hand-truck | `d4ee2ab94e08c6ac` | 7.443 | 7.696 | 0.836 | 0.128 |
| jogger | `ebb573cdf6466f0a` | 12.511 | 11.621 | 0.390 | 0.073 |
| kneeling-gardener | `d6c4d1236ca3532f` | 18.713 | 8.765 | 0.442 | 0.108 |
| leaning-person | `2cd99a547173aa3e` | 49.079 | 8.223 | 1.756 | 0.192 |
| person-cooking | `f0b1cc9f29841c74` | 52.925 | 8.191 | 1.557 | 0.161 |
| pregnant-person | `c0bafbfcbe7cfbb1` | 52.907 | 8.198 | 1.557 | 0.161 |
| runner-with-blade-prosthesis | `79cca1d56a8312a5` | 47.615 | 19.776 | 0.389 | 0.097 |
| seated-guitarist | `d86b96647aebbc70` | 57.894 | 22.497 | 1.033 | 0.135 |
| seated-office-worker | `e8d6b135349f7a95` | 56.003 | 19.833 | 0.970 | 0.157 |
| seated-reader | `92547aecd0033827` | 57.337 | 18.715 | 0.916 | 0.169 |
| skateboarder | `9e30636b16b73e84` | 51.736 | 0.031 | 0.468 | 0.107 |
| sleeping-person | `d83c849d3ac6a66d` | 66.826 | 8.687 | 2.225 | 0.169 |
| standing-adult | `8d6b852ff6aa2f8f` | 15.282 | 32.702 | 0.469 | 0.126 |
| walking-commuter | `546a8320e9a5f78d` | 28.375 | 20.062 | 1.689 | 0.168 |

The fixture and independent geometric oracle are `digital-human-shaft-case.json` and `digital-schoolchild-check.ts`. `digital-human-shaft-study.py` records the original sections, rebuilds the source-specific candidate and checks the installed geometry. Each entry uses its own existing 1:50 anatomical surfaces as the high-resolution reference. Corrected surfaces preserve the body’s joints, clip motion and equipment contacts.
