# Luna UI Popover Multi-Consumer Retention Audit

- Popover component SHA-256: `1045fb337d12c04688f41b58ec229ddc6246575aa6388259181bb3e959a45765`, clean.
- Popover story SHA-256: `a744f9872e535f6493db0f8bbce1104b7cb8218077881c4fe9820222bb32da7c`, clean.
- React index at audit time: `f4415689af8fadf41714bde7b4bc7181169804a7b878ee25411791ec8d5abf59`.

Independent production consumers are framework UI Search, ActionGroup, ToggleGroup, and protected renderer ShellSync/SyncAttachCard. The renderer package-index import is unused glue, while the Popover story and inline portal test are non-production evidence.

Decision: retain Popover at the framework UI owner. It exceeds the two-consumer threshold and the current owner is the lowest common owner across framework and product terminals. A raw package-level `PopoverPrimitive` namespace import in the shared React index is unused and queued for coordinator-only deletion; the direct dependency remains valid because the Popover component itself uses it. No component move follows.
