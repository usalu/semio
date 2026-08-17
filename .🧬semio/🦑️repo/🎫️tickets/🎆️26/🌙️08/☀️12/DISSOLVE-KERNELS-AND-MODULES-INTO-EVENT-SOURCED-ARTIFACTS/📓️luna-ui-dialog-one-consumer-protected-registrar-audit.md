# UI Dialog One-Consumer Protected-Registrar Audit

## Snapshot

- Definition: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/🟦️component.tsx`, SHA-256 `ed8b1511848bfe1247f860f20e1b879b7dbb39c4280dad97a0ce27372ca79c92`, clean.
- Story: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/🧪️story.tsx`, SHA-256 `ab383688381d4e90934afdae8cd6fd581b3b47751f8485ea8e094d4f36b21ae2`, clean.
- Shared React barrel SHA-256: `e82f73a9fd61e5d140d69f7df7498fa1afcd2217fde523fb6f64c9e130844e81`.

## Responsibility and Consumer Closure

- Dialog root/context, content/portal/close, modal presentation, title/description, and header/footer form one coherent compound Dialog concern rather than unrelated responsibilities.
- `Command` is the only active direct production consumer, using Dialog root, content, header, title, and description to implement `CommandDialog`.
- DialogTrigger and DialogFooter are story-only; DialogOverlay has no active direct consumer; DialogPortal and DialogClose are internal implementation helpers.
- ShellSearch and ShellHost consume `CommandDialog` transitively. Keeping the Command package API unchanged means they need no source edit.
- The protected OS renderer package index retains an unused Dialog glue import. It is not a consumer, but removing the UI package surface without its registrar update would leave a stale edge.

## Disposition

Queue one-consumer collapse: inline only the required compound Dialog behavior privately into Command, delete story-only/unused facets and the Dialog identity, keep `CommandDialog` behavior/API stable, and remove the UI barrel Dialog surface. Do not execute until the protected OS renderer package registrar owner releases or removes its exact unused Dialog edge. No ShellSearch or ShellHost behavior lease is necessary.
