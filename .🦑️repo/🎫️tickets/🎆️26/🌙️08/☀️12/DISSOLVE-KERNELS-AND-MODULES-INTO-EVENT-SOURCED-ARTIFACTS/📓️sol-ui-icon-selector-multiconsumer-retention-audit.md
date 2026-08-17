# UI Icon Selector Multiconsumer Retention Audit

## Baseline

- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- TypeScript component SHA-256: `02e1ff618fa0d8767f4e0021d65f56557e5a108d1e34a6de4f1963e7eb92ae51`
- Rust component SHA-256: `5490a6e76c5fc01bff58528015d6ff84e9a2a0140dcd0c6c2e76c15421be1f49`
- Story SHA-256: `61d61fe2867b94fed6bc9ec57dcefc4ba65d9b8a54d4df72612142c6c582f616`

## Responsibility

`🔣️IconSelector` is one specific icon-value editing interaction. Its private helpers decode an existing value for the selected representation, encode edited content, and deterministically reset incompatible representation changes. The component owns the mode picker, representation editor, file import, and preview required by that single interaction.

## Production Consumer Closure

Independent active production terminals include:

- protected OS renderer `ShellHelpers`, which renders it for icon schema fields;
- protected OS renderer `Interpreter`, which renders it for interpreted controls with classifier selection.

The framework React barrel and OS renderer package index are assembly/glue and do not count. The Storybook story is test/example provenance and does not count. The Rust wgpu leaf is a paired language implementation, not another production consumer.

## Disposition

Retain `🔣️IconSelector` as a specific UI component with its TypeScript and Rust language leaves co-owned. It has one coherent interaction responsibility and at least two independent production consumers. No split, module extraction, inlining, or deletion is justified.
