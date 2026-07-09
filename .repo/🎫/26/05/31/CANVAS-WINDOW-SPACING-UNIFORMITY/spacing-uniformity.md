# Shell Spacing Uniformity

All shell chrome gutters now use `--spacing-single` (1 ui-spacing unit), matching navbar/footer `p-single` / `gap-single`.

## Unified surfaces

| Surface                            | Before           | After            |
| ---------------------------------- | ---------------- | ---------------- |
| Mode canvas inset                  | `p-double`       | `p-single`       |
| Canvas container                   | `p-double`       | `p-single`       |
| Horizontal/vertical window stacks  | `gap-double`     | `gap-single`     |
| Resizable split handles            | `spacing-double` | `spacing-single` |
| Side panel / floating panel offset | `spacing-double` | `spacing-single` |
| Touch toolbar gap/padding          | `spacing-double` | `spacing-single` |
| Toolbar group gap                  | `spacing-double` | `spacing-single` |

Navbar, footer, band, strip, and window engagement overlay already used `single`.
