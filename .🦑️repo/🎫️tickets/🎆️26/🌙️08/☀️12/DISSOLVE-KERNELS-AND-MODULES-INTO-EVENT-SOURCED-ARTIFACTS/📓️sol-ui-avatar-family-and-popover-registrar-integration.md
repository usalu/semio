# UI Avatar Family and Popover Registrar Integration

## Preconditions

- Shared React registrar matched the leased pre-edit SHA-256 `f4415689af8fadf41714bde7b4bc7181169804a7b878ee25411791ec8d5abf59`.
- Terra completed the Avatar-family source move without editing the registrar.
- The Popover multiconsumer audit separately proved the registrar's package-level `PopoverPrimitive` namespace import had no use.

## Registrar Changes

- Removed the broad old Avatar import/export surface (`Avatar`, `AvatarImage`, `AvatarFallback`, `DraggableAvatar`, and their obsolete contracts).
- Registered only `TableAvatar` and `TableAvatarProps` from the specific `📻️TableAvatar` component.
- Removed the package-level `AvatarPrimitive` namespace import; the Radix adapter now stays private in `📻️TableAvatar`.
- Removed the unused package-level `PopoverPrimitive` namespace import; the retained shared Popover implementation owns its Radix adapter privately.

## Validation

- Final registrar SHA-256: `fdd7e8ec24ea5288b386bab04f2627d81194712e2461860e8e2abcead71a4a23`.
- Registrar scans contain no `AvatarPrimitive`, `PopoverPrimitive`, old Avatar path, `DraggableAvatar`, `AvatarImage`, or `AvatarFallback` residue.
- The registrar has exactly the specific `TableAvatar` import/export region.
- Ordinary and cached scoped `git diff --check` both passed.
