# App Command Scope Classification Audit

The real CAD constructor demonstrated that an action-only classifier does not update an app-level command. The coordinator inspected the builder: `action_interactive_job` searches only `self.actions`; `interactive_jobs` separately handles actions and commands. The generated EditorBuilder forwarder calls that same method. Textual presence of an action classification is therefore not proof of a command's generated execution semantics.

A scoped all-plugin search for setContributions found:

| App | Actual command declaration | Action-only override | Assessment |
| --- | --- | --- | --- |
| CAD | Newly explicitly Migrated | Removed by current repair | Real constructor rerun pending |
| Playbook | Plain bounded_catalog, no explicit Migrated | Migrated | Same source mismatch; exact Config factory exists, repair assigned |
| Process3d | Explicitly Migrated on the command | Redundant Migrated | No same command-classification defect; constructor not run here |
| Sourcing Curate | Explicitly Migrated on the command | Redundant Migrated | No same command-classification defect; constructor not run here |

Flow and Imperative still explicitly use BatchOnly action overrides; this audit does not promote them. All command-scope repairs must preserve the concrete factory/owner/controller/schema join and have language-neutral route tests plus actual activation coverage. The source inventory must not count a similarly spelled action declaration as a runtime command proof.

The two duplicate rows in the current source census were separately inspected: Writer's `setEditorSetting` is three distinct wire variants (`font-px`, `line-height`, `tab-size`) sharing one manifest action ID. They are not two duplicate runtime registrations demonstrated by this audit. Their existing exact dispatch/field semantics still require the normal app gates.

Production changes are executor-owned; the coordinator performed read-only inspection and dispatch. See `📓️coordinator-cad-constructor-r1-review-2026-08-27.md` and `📓️coordinator-remaining-app-command-cohorts-2026-08-27.md` for the surrounding all-app work.
