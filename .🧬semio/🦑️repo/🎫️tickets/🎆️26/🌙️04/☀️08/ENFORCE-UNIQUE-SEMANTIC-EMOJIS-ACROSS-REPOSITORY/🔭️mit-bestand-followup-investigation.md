# Mit-Bestand Follow-Up Investigation

Read-only investigation on 2026-09-05. No Mit-Bestand files were renamed, rewritten, removed, regenerated, or restored from Git during this investigation.

## Why The Inventory Grew

The prior `♻️mit-bestand-repair.md` covered 221 files and 250 governed entries. The latest unscoped snapshot now contains **1,598 Mit-Bestand findings: 23 missing emoji identities and 1,575 duplicate sibling identities**.

These are newly introduced report asset copies and report sources, not the disappearance of the earlier handpicked Zwischenbericht project/logo names:

- Both report actor trees contain **752 regular files** in eleven unprefixed country directories. Their relative names and SHA-256 values match **752/752** between Zwischenbericht and Forschungsbericht. Most filenames retain the same `🖼️` carrier prefix for every sibling. Example AT `🖼️F01.png` has distinct inodes and link count one in the two trees; these are copies, not symlinks or hardlinks.
- The Forschungsbericht project tree contains **67 files** matching all 67 Zwischenbericht files by emoji-stripped coordinate and SHA-256, but only **one** matches the already repaired full coordinate. The new copy therefore carries old repeated folder identities while the prior canonical Zwischenbericht names remain intact.
- The Forschungsbericht logo tree contains **10 files** matching the prior ten logos by emoji-stripped coordinate and SHA-256; **none** shares the earlier full semantic basename. Kompaktbericht also has a copied generic logo tree.
- The two report directories `📑️forschungsbericht` and `📝️kompaktbericht`, and the Zwischenbericht actor asset tree, are currently untracked additions. The report router has 805 added and 44 removed lines relative to HEAD, with new document routes and actor-network rendering/validation source. This is substantive concurrent report work and must be preserved.
- Filesystem birth/modification times for the inspected actor copies are 04:45:03 (Forschungsbericht) and 04:45:09 (Zwischenbericht), after the prior repaired asset directories (23:52). The present report sources were updated subsequently. Timestamp evidence supports newly copied content; it does not identify the agent responsible.

## Finding Breakdown

| Branch | Findings |
| --- | ---: |
| Zwischenbericht actor tree | 752 |
| Forschungsbericht actor tree | 752 |
| Forschungsbericht project tree | 66 |
| Forschungsbericht appendices | 9 |
| Forschungsbericht bare demonstrator directory | 1 |
| Forschungsbericht logo tree | 9 |
| Kompaktbericht logo tree | 9 |
| Total | 1,598 |

The actor trees use the country roster AT, BE, CH, DE, DK, FI, FR, GB, NL, NO, SE. Their actor basenames are identifiers such as F01/I01/M01/N01/U01 with optional `-dark`, so an arbitrary emoji palette cannot express their meaning. A proper follow-up must use the actor ledger/entity names and explicitly distinguish each light/dark pair, then repair the corresponding `assetPath` references and rendering source. Existing canonical project/logo choices can be individually reused for the byte-identical copies without inventing new identities.

## Integration Caution

The new report router explicitly owns `DOCUMENTS`, `ACTOR_NETWORK_LEDGER`, `ACTOR_NETWORK_FIGURES`, `ACTOR_NETWORK_TABLES`, `ACTOR_NETWORK_PROGRAMS`, and `ACTOR_NETWORK_INTRO`. It checks each actor's `assetPath` against the Forschungsbericht tree. Any further repair must coordinate exact path changes with these authored references, not merely rename directories, hide the copies behind exclusions, reset the work, or apply a generated discriminator palette.
