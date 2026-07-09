# Verify Log — Print Uniform Block Spacing

## Window before-spacing fix

Windows (`Table`, `Figure`, `Window`, …) were glued to preceding body text because only **after-close** block sep was applied.

### Model
- `\semio_block_sep_before:` — one `\semio@block@sep` unit before a window opens, unless spacing was already emitted
- `\semio_block_sep_after:` — same unit after close, sets `\ifsemio@block@sep@done`
- Heading row wrap sets `\semio@block@sep@done` after its trailing `\vskip` so **section→table** does not double-gap

### Build
```bash
bun ./script.ts build paper
```
