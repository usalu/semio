# Terra Packet: Command Separator Zero-Consumer Deletion

## Objective and Lease

Delete the private, zero-consumer `CommandSeparator` function from `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⌨️Command/🟦️component.tsx`. Preserve every public command wrapper, story surface, import, and behavior. The only other writable path is one unique ticket acceptance Markdown.

Baseline SHA-256: `a42551eb3cf50b3b1284db3ce9c7f2afb900ddc787434abc2cef486c90f09b3e`.

## Gates

- Active source scan has zero `CommandSeparator`.
- Component export set is unchanged.
- Scoped ordinary/cached `git diff --check` pass.
- Run UI React lint and test-quick once through Nx with `--skip-nx-cache`.
- Record final SHA and exact gate classification; do not edit any barrel/product/lock/generated path.
