# Taxonomy Discovery

SSOT: 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json

## Registration

Add exact domain semanticDirectoryKinds entries with emoji, anchored slugPattern, allowEmojiOnly false, parentKindIds [teaching-architecture]; append physical names to semanticDirectoryMemberKinds members-of-teaching-architecture.memberNames. Add exact topic kinds with each domain parent and a members-of-teaching-DOMAIN roster with ownerKindIds and memberNames. Extend teaching-quiz.parentKindIds and members-of-teaching-topics.ownerKindIds with all new topic kinds; existing members-of-teaching-topics.memberNames already admits ❓️quiz and 🎬️clip.

Generic tests and fixtures kinds already exist. Add exact curriculum directory names to members-of-tests.memberNames and members-of-fixtures.memberNames, or a narrow shared member kind with ownerKindIds [tests, fixtures]. Fixtures has no memberSlugPattern, so explicit registration matters.

No mockup or mockups directory kind/member exists. Add exact mockups facet and narrow children member roster; parent scope should be the framework course owner when available. Keep JSON/HTML kind-only filenames inside semantic concern children.

## Safe Inspection

Use repo tools exec --light node for bounded fs reads and JSON inspection. Normal node reserves 2048 MB and may wait on current machine load. Do not use native folder list or full recursive scans. Parse latest JSON before targeted object mutation so concurrent changes survive. No registration was edited during discovery.
