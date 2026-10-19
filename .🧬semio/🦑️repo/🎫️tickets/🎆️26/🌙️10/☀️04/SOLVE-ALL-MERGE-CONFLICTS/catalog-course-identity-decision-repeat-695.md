# Course Catalog Identity Decision — Repeat 695

Retire only the obsolete framework.product.quiz metadata row. Its actual receiving identity is framework.product.course, already recorded at the current Course schema owner. No old API alias or false schema identity is introduced.

- B and H contain framework.product.quiz, and neither contains framework.product.course. S contains framework.product.course and omits framework.product.quiz.
- The old Quiz JSON schema in H is byte-for-byte B, so there is no later H-only schema feature delta to recover.
- All **88 H definitions/exports** have current Course counterparts. Catalog → Course, CatalogQuizView → CourseQuizView and CatalogView → CourseView are the only renamed export identities. The other 85 retain their names.
- **73 definitions** remain structurally identical. The other 15 contain incoming authored product/wire/presence extensions and Course terminology. CourseQuizView is identical to CatalogQuizView; Course and CourseView preserve prior properties and add topics, while adopting the incoming semio.course/v1 discriminator.
- The current Course JSON source is exact immutable S bytes. Current declared identity: https://json.schemas.assets.semio-tech.com/framework/product/course/schema.json.
- The old Quiz source files are absent; assigning the old Quiz scope ID to Course would contradict the declared current schema $id.
- Earlier independent domain source relocation audit accounts for the 44 H-only Quiz child/source migrations into Course. The retained deployment configuration is independently covered by course-docker-cargo-accounting-repeat-695.md.

This semantic retirement will occur in the final single guarded catalog refresh, together with nine live H metadata rows restored, four proven Lowpoly owner paths corrected and raw source hashes refreshed. No catalog mutation occurred while preparing this evidence.
