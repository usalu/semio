# Material Five Provenance Direct Owner Map

Read-only static direct-owner trace and independent rawbytes hashing; no Source runtime evaluation or materialexecution. This resolves earlier boundedscan limitation.

Actual controller material argv is repo run @teaching/proctor:check --configuration=release -- /repo/🎓️teaching/🏛️architecture/📚️course/🔣️.json. Current registry project check target dependsOn build, routes through BunCheckScript→runProctor(...,[check,coursepath]); bootstrapRust calls proctor::cli::run; CLIcheck invokes teaching_proctor_base::course::load_course.

Actual output is plain text, not a separate closedJSON materialreceipt: course line fields courseid,path,quizcount,badgecount,fingerprint; fiveordered quiz lines carry quizid,taskcount,entryrelativepath,revision. All five revision fields are lowercase SHA256of RAWquizfilebytes. Loader aggregate fingerprint is SHA256 over exactdomainUTF8 `semio/teaching/proctor/course/v1` plusNUL, rawcoursebytes, then for eachorderedcourseentry:UTF8entrypath,NUL,ASCIIhexquizrevision. This is distinct from normativewire8/FNVcontractfingerprint. Independentcrypto implementation yields currentcoursefingerprint `e9aadcf92857930586f0dbb2f039c6b3982006f84cff5422a8e1055a8980e631`.

| Quiz | Tasks | Raw File Revision |
| --- | ---: | --- |
| physics | 3 | `1795e5c9491d9f46f5bd591e4e10d45faea6e13188a61522e913dc811a5bae26` |
| ventilation | 3 | `9f256c7b64ca73c124633d320d76632424ec562713546ee6fa9abf53fe46762d` |
| heating | 2 | `db04b89a6218b9296f2f129290c0d675562cac470f7b46e8d30749e2ffc5a319` |
| cooling | 1 | `82320ffc1602ca2f2c29bdf768c59ab490acfd4c68a9469329e3c00350ba55c4` |
| demand | 2 | `73da95631f43d2034bc2911178e50575ad66c2cc1de47b3a4d1364d8c23b1f4c` |

Loader validates course_issues plus everyquiz_issues, sorts/deduplicates issues, refuses wholecourse before success; rawquiz entries retain source/path/quiz/revision. Material stdout is retained in actual controller completeoutput and exactstep0 proof, not a separate productreceipt fingerprint contract. No missing required materialoutputschema is inferred: currentowner intentionally emits text; fiveprovenance lines and aggregate can be checked against actual output after execution.

Executableprovenance is separate actual productreceipt closed10fields:version,package,binary,mode,platform,architecture,executable,sha256,bytes,sources. Productacquisition validates receipt, exactmode/hostplatform+architecture, ordinary executable,size andfullSHA256 before private artifactlaunch under actualOriginal. Source list belongs to productreceipt capture/build provenance; checkProctorProduct does not freshly rehash allSource records. Do not claim these checks alone prove currentruntime material success; current Native/Cargo taskorchestration and retained executable supply actualbuild authority.

Exact directpins:

- `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json`: 13137B / `e80faaf096c07348e838b07d9af7f9198760709c26d19a7535c92f0ae1ea918c`
- `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📜️script.ts`: 8935B / `5cbf2c7bd3d71820742d0d9b4a57f9cfa8fe24b94dc75d75538b2233d7e2cddd`
- `🎓️teaching/🛂️proctor/🏗️bootstrap/🦀️.rs`: 663B / `756432be61b925f4f410be0732d05ebb4cc47b650cf0c5103712213546637b01`
- `🎓️teaching/🛂️proctor/🔨️modules/⌨️cli/🦀️.rs`: 37362B / `cd819758961e4c521c5e29f808637cf405e36874eabf2a07cfcc13634ca02086`
- `🎓️teaching/🛂️proctor/🔨️modules/📚️course/🦀️.rs`: 6695B / `434e46b34e1faf5999c28b970d0bd806d5758ce14ac015baa90daa84a47f626a`
- `🎓️teaching/🛂️proctor/🏗️bootstrap/🧾️product/🟦️.ts`: 33851B / `97d7bee181eeb1e4b3d52d01618ec42c36ed2106ac17773c985951eead0d724b`
- `🎓️teaching/🛂️proctor/🏗️bootstrap/🧾️product/🧬️schema/🔣️.json`: 3930B / `bd9a1d551fad271d5317ec28162db59303c8bb9e425e92d789118e2b980b59cb`
