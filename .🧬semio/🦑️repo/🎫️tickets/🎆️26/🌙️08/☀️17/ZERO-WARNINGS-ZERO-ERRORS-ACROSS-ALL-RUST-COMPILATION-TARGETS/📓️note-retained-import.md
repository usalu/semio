# Note Retained Import

Pass 342 removes the unused HistoryView import from Note's retained command module. Full native307 reported this same warning for both library and test compilation. A raw source search confirmed the import was its only remaining reference after the shared retained-command input refactor.

Changed: `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs`.

The initially prepared pass341 script never ran: Nx project-graph construction failed because the concurrently changing Print project referenced a missing tectonic-template-compilation script. The small source edit was applied with the editor patch tool; compilation and tests continue to use Bun/Nx. No unrelated Print files were changed. Fresh native339 verification remains required.
