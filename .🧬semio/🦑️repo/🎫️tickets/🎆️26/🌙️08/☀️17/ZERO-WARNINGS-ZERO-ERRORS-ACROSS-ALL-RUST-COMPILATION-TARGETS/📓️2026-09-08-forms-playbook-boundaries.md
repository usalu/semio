# Forms and Playbook Boundaries

Pass577 removes artifact declarations' dependency on their former parent-plugin application enums. Forms and Playbook now each declare the precise application trait required by their existing editor/viewer surfaces, with blanket implementations for matching plugin applications. Their artifact, standard and subset constructors carry the generic application type throughout. Concrete editor/viewer types, codec registrations, examples and schema descriptors are unchanged.

Files:

- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🦀️.rs`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🦀️.rs`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs`

These changes address native560's missing FormsApps and PlaybookApps errors. Syntax, compiler and runtime checks remain pending.
