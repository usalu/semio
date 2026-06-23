# All Hardcoded `"composerepo://` URI Strings in `main_test.go`

Total: **144 occurrences** across **18 test functions**

---

## 1. `TestSectionIdentificationAutofix` (line 3474)

| #   | Line | URI                    | Code                                                           |
| --- | ---- | ---------------------- | -------------------------------------------------------------- |
| 1   | 3510 | `composerepo://section/` | `if !strings.Contains(fixedContent, "composerepo://section/") {` |

---

## 2. `TestDefinitionIdentificationAutofix` (line 3515)

| #   | Line | URI                       | Code                                                              |
| --- | ---- | ------------------------- | ----------------------------------------------------------------- |
| 2   | 3551 | `composerepo://definition/` | `if !strings.Contains(fixedContent, "composerepo://definition/") {` |

---

## 3. `TestDefinitionNativeDocstringAutofix` (line 3848)

| #   | Line | URI                       | Code                                                              |
| --- | ---- | ------------------------- | ----------------------------------------------------------------- |
| 3   | 3896 | `composerepo://definition/` | `if !strings.Contains(fixedContent, "composerepo://definition/") {` |

---

## 4. `TestPythonTripleQuoteDocstringAutofix` (line 3901)

| #   | Line | URI                       | Code                                                              |
| --- | ---- | ------------------------- | ----------------------------------------------------------------- |
| 4   | 3943 | `composerepo://definition/` | `if !strings.Contains(fixedContent, "composerepo://definition/") {` |

---

## 5. `TestPythonTripleQuoteDocstringMerge` (line 3954)

| #   | Line | URI                       | Code                                                              |
| --- | ---- | ------------------------- | ----------------------------------------------------------------- |
| 5   | 3996 | `composerepo://definition/` | `if !strings.Contains(fixedContent, "composerepo://definition/") {` |

---

## 6. `TestFormatHeaderStructure` (line 4493)

| #   | Line | URI                             | Code                                                                                                                                                                             |
| --- | ---- | ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 6   | 4495 | `composerepo://file/test/file.ts` | `header := lang.FormatHeader("💻test/file.ts", "composerepo://file/test/file.ts", "A test file", "2025 Test User <test@test.com>", "AGPL license text here", "Some requirements")` |

---

## 7. `TestFormatHeaderEmptyRequirements` (line 4519)

| #   | Line | URI                             | Code                                                                                                                            |
| --- | ---- | ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| 7   | 4521 | `composerepo://file/test/file.go` | `header := lang.FormatHeader("💻test/file.go", "composerepo://file/test/file.go", "", "2025 Dev <dev@dev.com>", "AGPL text", "")` |

---

## 8. `TestFormatHeaderAllLanguages` (line 4530)

| #   | Line | URI                          | Code                                                                                                             |
| --- | ---- | ---------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| 8   | 4543 | `composerepo://file/test/file` | `header := lang.FormatHeader("💻test/file", "composerepo://file/test/file", "", "2025 Dev <d@d.com>", "AGPL", "")` |
| 9   | 4557 | `composerepo://file/test/file` | `header := lang.FormatHeader("💻test/file", "composerepo://file/test/file", "", "2025 Dev <d@d.com>", "AGPL", "")` |

---

## 9. `TestTerritory` (line 4593)

| #   | Line | URI                      | Code                                                     |
| --- | ---- | ------------------------ | -------------------------------------------------------- |
| 10  | 4716 | `composerepo://territory/` | `if !strings.HasPrefix(uri, "composerepo://territory/") {` |

---

## 10. `TestCliE2E_TicketLifecycle_Syntaxes_NoManagement` (line 7310)

| #   | Line | URI                 | Code                                                      |
| --- | ---- | ------------------- | --------------------------------------------------------- |
| 11  | 7350 | `composerepo://file/` | `fileURI := "composerepo://file/" + PathToUriPath(fileRel)` |

---

## 11. `TestRenderEntityMarkdownLink_AllKinds` (line ~8740)

| #   | Line | URI            | Code                                             |
| --- | ---- | -------------- | ------------------------------------------------ |
| 12  | 8763 | `composerepo://` | `if !strings.Contains(output, "composerepo://") {` |

---

## 12. `TestArtifactIDAndURI` (line ~9580)

| #   | Line | URI                                                                                                | Code                                                                                                           |
| --- | ---- | -------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| 13  | 9601 | `composerepo://root`                                                                                 | `wantURI: "composerepo://root",`                                                                                 |
| 14  | 9608 | `composerepo://projects`                                                                             | `wantURI: "composerepo://projects",`                                                                             |
| 15  | 9615 | `composerepo://project/compose`                                                                        | `wantURI: "composerepo://project/compose",`                                                                        |
| 16  | 9622 | `composerepo://project/repo`                                                                         | `wantURI: "composerepo://project/repo",`                                                                         |
| 17  | 9629 | `composerepo://project/coda`                                                                         | `wantURI: "composerepo://project/coda",`                                                                         |
| 18  | 9636 | `composerepo://bundles`                                                                              | `wantURI: "composerepo://bundles",`                                                                              |
| 19  | 9643 | `composerepo://bundle/compose/js`                                                                      | `wantURI: "composerepo://bundle/compose/js",`                                                                      |
| 20  | 9650 | `composerepo://bundle/coda/examples`                                                                 | `wantURI: "composerepo://bundle/coda/examples",`                                                                 |
| 21  | 9657 | `composerepo://bundle/compose/desktop`                                                                 | `wantURI: "composerepo://bundle/compose/desktop",`                                                                 |
| 22  | 9664 | `composerepo://folders`                                                                              | `wantURI: "composerepo://folders",`                                                                              |
| 23  | 9671 | `composerepo://folders/compose/js/src`                                                                 | `wantURI: "composerepo://folders/compose/js/src",`                                                                 |
| 24  | 9678 | `composerepo://folder/compose/js/src`                                                                  | `wantURI: "composerepo://folder/compose/js/src",`                                                                  |
| 25  | 9685 | `composerepo://folder/compose/js/utils`                                                                | `wantURI: "composerepo://folder/compose/js/utils",`                                                                |
| 26  | 9692 | `composerepo://files`                                                                                | `wantURI: "composerepo://files",`                                                                                |
| 27  | 9699 | `composerepo://file/test.txt`                                                                        | `wantURI: "composerepo://file/test.txt",`                                                                        |
| 28  | 9706 | `composerepo://file/main.go`                                                                         | `wantURI: "composerepo://file/main.go",`                                                                         |
| 29  | 9713 | `composerepo://file/compose/js/src/index.test.ts`                                                      | `wantURI: "composerepo://file/compose/js/src/index.test.ts",`                                                      |
| 30  | 9720 | `composerepo://file/tsconfig.json`                                                                   | `wantURI: "composerepo://file/tsconfig.json",`                                                                   |
| 31  | 9727 | `composerepo://file/build.sh`                                                                        | `wantURI: "composerepo://file/build.sh",`                                                                        |
| 32  | 9734 | `composerepo://file/logo.png`                                                                        | `wantURI: "composerepo://file/logo.png",`                                                                        |
| 33  | 9741 | `composerepo://file/LICENSE.md`                                                                      | `wantURI: "composerepo://file/LICENSE.md",`                                                                      |
| 34  | 9748 | `composerepo://sections/compose/js/src/index.ts`                                                       | `wantURI: "composerepo://sections/compose/js/src/index.ts",`                                                       |
| 35  | 9755 | `composerepo://section/compose/js/src/Design.tsx/State%20Management/Design%20Store`                    | `wantURI: "composerepo://section/compose/js/src/Design.tsx/State%20Management/Design%20Store",`                    |
| 36  | 9762 | `composerepo://section/compose/js/src/file.ts/Imports`                                                 | `wantURI: "composerepo://section/compose/js/src/file.ts/Imports",`                                                 |
| 37  | 9769 | `composerepo://definitions/compose/js/src/index.ts`                                                    | `wantURI: "composerepo://definitions/compose/js/src/index.ts",`                                                    |
| 38  | 9776 | `composerepo://definition/compose/js/src/index.ts/MyClass`                                             | `wantURI: "composerepo://definition/compose/js/src/index.ts/MyClass",`                                             |
| 39  | 9783 | `composerepo://definition/compose/js/src/file.ts/Types/MyInterface`                                    | `wantURI: "composerepo://definition/compose/js/src/file.ts/Types/MyInterface",`                                    |
| 40  | 9790 | `composerepo://definition/repo/cli/main.go/GraphQL%20Types/GraphQL%20Input%20Types/TicketCloseInput` | `wantURI: "composerepo://definition/repo/cli/main.go/GraphQL%20Types/GraphQL%20Input%20Types/TicketCloseInput",` |
| 41  | 9797 | `composerepo://definition/compose/js/src/file.ts/MAX_SIZE`                                             | `wantURI: "composerepo://definition/compose/js/src/file.ts/MAX_SIZE",`                                             |
| 42  | 9804 | `composerepo://tickets`                                                                              | `wantURI: "composerepo://tickets",`                                                                              |
| 43  | 9816 | `composerepo://ticket/2025/02/04/test-ticket`                                                        | `wantURI: "composerepo://ticket/2025/02/04/test-ticket",`                                                        |
| 44  | 9829 | `composerepo://ticket/2025/02/04/test-ticket`                                                        | `wantURI: "composerepo://ticket/2025/02/04/test-ticket",`                                                        |
| 45  | 9836 | `composerepo://goals`                                                                                | `wantURI: "composerepo://goals",`                                                                                |
| 46  | 9843 | `composerepo://goal/RUNNING-SKETCHPAD`                                                               | `wantURI: "composerepo://goal/RUNNING-SKETCHPAD",`                                                               |
| 47  | 9850 | `composerepo://goal/R26-02/RUNNING-SKETCHPAD`                                                        | `wantURI: "composerepo://goal/R26-02/RUNNING-SKETCHPAD",`                                                        |
| 48  | 9857 | `composerepo://drafts`                                                                               | `wantURI: "composerepo://drafts",`                                                                               |
| 49  | 9864 | `composerepo://draft/my-draft`                                                                       | `wantURI: "composerepo://draft/my-draft",`                                                                       |
| 50  | 9871 | `composerepo://todos`                                                                                | `wantURI: "composerepo://todos",`                                                                                |
| 51  | 9878 | `composerepo://todo/my-todo`                                                                         | `wantURI: "composerepo://todo/my-todo",`                                                                         |
| 52  | 9885 | `composerepo://policies`                                                                             | `wantURI: "composerepo://policies",`                                                                             |
| 53  | 9892 | `composerepo://policy/code-hygiene`                                                                  | `wantURI: "composerepo://policy/code-hygiene",`                                                                  |
| 54  | 9899 | `composerepo://statutes`                                                                             | `wantURI: "composerepo://statutes",`                                                                             |
| 55  | 9906 | `composerepo://statute/code/inline-comment`                                                          | `wantURI: "composerepo://statute/code/inline-comment",`                                                          |
| 56  | 9913 | `composerepo://contributors`                                                                         | `wantURI: "composerepo://contributors",`                                                                         |
| 57  | 9920 | `composerepo://contributor/usalu`                                                                    | `wantURI: "composerepo://contributor/usalu",`                                                                    |
| 58  | 9927 | `composerepo://commits`                                                                              | `wantURI: "composerepo://commits",`                                                                              |
| 59  | 9934 | `composerepo://commit/abc123`                                                                        | `wantURI: "composerepo://commit/abc123",`                                                                        |
| 60  | 9941 | `composerepo://interactions`                                                                         | `wantURI: "composerepo://interactions",`                                                                         |
| 61  | 9948 | `composerepo://interaction/on/ticket/introduceinteractionmechanism/started`                          | `wantURI: "composerepo://interaction/on/ticket/introduceinteractionmechanism/started",`                          |
| 62  | 9955 | `composerepo://interaction/on/goal/r2602/finished`                                                   | `wantURI: "composerepo://interaction/on/goal/r2602/finished",`                                                   |

---

## 13. `TestIdToUri` (line ~9965)

| #   | Line  | URI                                                               | Code                                                                                                                                      |
| --- | ----- | ----------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| 63  | 9979  | `composerepo://project/compose`                                       | `{"project user", emojiText(EmojiProjectUser) + "compose", "composerepo://project/compose"},`                                                   |
| 64  | 9980  | `composerepo://project/composerepo`                                   | `{"project infra", emojiText(EmojiProjectInfra) + "composerepo", "composerepo://project/composerepo"},`                                         |
| 65  | 9981  | `composerepo://bundle/compose/js`                                     | `{"bundle", emojiText(EmojiProjectUser) + "compose" + emojiText(EmojiBundleLibrary) + "js", "composerepo://bundle/compose/js"},`                |
| 66  | 9982  | `composerepo://folder/src`                                          | `{"folder required", emojiText(EmojiFolderRequired) + "src", "composerepo://folder/src"},`                                                  |
| 67  | 9983  | `composerepo://folder/utils`                                        | `{"folder org", emojiText(EmojiFolderOrg) + "utils", "composerepo://folder/utils"},`                                                        |
| 68  | 9984  | `composerepo://file/testtxt`                                        | `{"file docs", emojiText(EmojiFileDocs) + "testtxt", "composerepo://file/testtxt"},`                                                        |
| 69  | 9985  | `composerepo://file/maingo`                                         | `{"file code", emojiText(EmojiFileCode) + "maingo", "composerepo://file/maingo"},`                                                          |
| 70  | 9986  | `composerepo://sections`                                            | `{"section collection", emojiText(EmojiSection), "composerepo://sections"},`                                                                |
| 71  | 9987  | `composerepo://section/compose/js/src/designtsx/statemanagment/store` | `{"section", buildSectionID(...), "composerepo://section/compose/js/src/designtsx/statemanagment/store"},`                                    |
| 72  | 9988  | `composerepo://definition/compose/js/src/filets/types/myclass`        | `{"definition impl", buildDefinitionID(...), "composerepo://definition/compose/js/src/filets/types/myclass"},`                                |
| 73  | 9989  | `composerepo://tickets`                                             | `{"ticket collection", emojiText(EmojiTicket), "composerepo://tickets"},`                                                                   |
| 74  | 9990  | `composerepo://ticket/testticket`                                   | `{"ticket", emojiText(EmojiTicket) + "testticket", "composerepo://ticket/testticket"},`                                                     |
| 75  | 9991  | `composerepo://goals`                                               | `{"goal collection", emojiText(EmojiGoal), "composerepo://goals"},`                                                                         |
| 76  | 9992  | `composerepo://goal/r2602runningsketchpad`                          | `{"goal", emojiText(EmojiGoal) + "r2602runningsketchpad", "composerepo://goal/r2602runningsketchpad"},`                                     |
| 77  | 9993  | `composerepo://goal/r2602/runningsketchpad`                         | `{"goal nested", emojiText(EmojiGoal) + "r2602" + emojiText(EmojiGoal) + "runningsketchpad", "composerepo://goal/r2602/runningsketchpad"},` |
| 78  | 9994  | `composerepo://drafts`                                              | `{"draft collection", emojiText(EmojiDraft), "composerepo://drafts"},`                                                                      |
| 79  | 9995  | `composerepo://draft/mydraft`                                       | `{"draft", emojiText(EmojiDraft) + "mydraft", "composerepo://draft/mydraft"},`                                                              |
| 80  | 9996  | `composerepo://policies`                                            | `{"policy collection", emojiText(EmojiPolicy), "composerepo://policies"},`                                                                  |
| 81  | 9997  | `composerepo://policy/codehygiene`                                  | `{"policy", emojiText(EmojiPolicy) + "codehygiene", "composerepo://policy/codehygiene"},`                                                   |
| 82  | 9998  | `composerepo://contributors`                                        | `{"contributor collection", emojiText(EmojiContributor), "composerepo://contributors"},`                                                    |
| 83  | 9999  | `composerepo://contributor/usalu`                                   | `{"contributor", emojiText(EmojiContributor) + "usalu", "composerepo://contributor/usalu"},`                                                |
| 84  | 10000 | `composerepo://commits`                                             | `{"commit collection", emojiText(EmojiCommit), "composerepo://commits"},`                                                                   |
| 85  | 10001 | `composerepo://commit/abc123`                                       | `{"commit", emojiText(EmojiCommit) + "abc123", "composerepo://commit/abc123"},`                                                             |
| 86  | 10002 | `composerepo://interaction/on/ticket/testticket/started`            | `{"interaction started ticket", ..., "composerepo://interaction/on/ticket/testticket/started"},`                                            |
| 87  | 10003 | `composerepo://interaction/on/goal/r2602/finished`                  | `{"interaction finished goal", ..., "composerepo://interaction/on/goal/r2602/finished"},`                                                   |

---

## 14. `TestUriToId` (line ~10010)

| #   | Line  | URI                                                                             | Code                                                                                                                                                          |
| --- | ----- | ------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 88  | 10022 | `composerepo://root`                                                              | `{"repo", "composerepo://root", ""},`                                                                                                                           |
| 89  | 10023 | `composerepo://projects`                                                          | `{"projects", "composerepo://projects", emojiText(EmojiProjects)},`                                                                                             |
| 90  | 10024 | `composerepo://project/compose`                                                     | `{"project", "composerepo://project/compose", emojiText(EmojiProjectUser) + "compose"},`                                                                            |
| 91  | 10025 | `composerepo://project/repo`                                                      | `{"project infra", "composerepo://project/repo", emojiText(EmojiProjectInfra) + "composerepo"},`                                                                  |
| 92  | 10026 | `composerepo://bundles`                                                           | `{"bundles", "composerepo://bundles", emojiText(EmojiBundles)},`                                                                                                |
| 93  | 10027 | `composerepo://bundle/compose/js`                                                   | `{"bundle", "composerepo://bundle/compose/js", emojiText(EmojiProjectUser) + "compose" + emojiText(EmojiBundleLibrary) + "js"},`                                    |
| 94  | 10028 | `composerepo://folders`                                                           | `{"folders", "composerepo://folders", emojiText(EmojiFolders)},`                                                                                                |
| 95  | 10029 | `composerepo://folders/compose/js/src`                                              | `{"folders with parent", "composerepo://folders/compose/js/src", emojiText(EmojiFolders)},`                                                                       |
| 96  | 10030 | `composerepo://folder/compose/js/src`                                               | `{"folder", "composerepo://folder/compose/js/src", emojiText(EmojiFolderOrg) + "composejssrc"},`                                                                    |
| 97  | 10031 | `composerepo://files`                                                             | `{"files", "composerepo://files", emojiText(EmojiFiles)},`                                                                                                      |
| 98  | 10032 | `composerepo://file/test.txt`                                                     | `{"file", "composerepo://file/test.txt", emojiText(EmojiFileCode) + "testtxt"},`                                                                                |
| 99  | 10033 | `composerepo://sections`                                                          | `{"sections", "composerepo://sections", emojiText(EmojiSections)},`                                                                                             |
| 100 | 10034 | `composerepo://section/compose/js/src/Design.tsx/State%20Management/Design%20Store` | `{"section", "composerepo://section/compose/js/src/Design.tsx/State%20Management/Design%20Store", buildSectionID(...)},`                                          |
| 101 | 10035 | `composerepo://definitions`                                                       | `{"definitions", "composerepo://definitions", emojiText(EmojiDefinitions)},`                                                                                    |
| 102 | 10036 | `composerepo://definition/compose/js/src/file.ts/myFunc`                            | `{"definition single", "composerepo://definition/compose/js/src/file.ts/myFunc", buildDefinitionID(...)},`                                                        |
| 103 | 10037 | `composerepo://definition/compose/js/src/file.ts/Section/myFunc`                    | `{"definition with section", "composerepo://definition/compose/js/src/file.ts/Section/myFunc", buildDefinitionID(...)},`                                          |
| 104 | 10038 | `composerepo://tickets`                                                           | `{"tickets", "composerepo://tickets", emojiText(EmojiTicket)},`                                                                                                 |
| 105 | 10039 | `composerepo://ticket/2025/02/04/test-ticket`                                     | `{"ticket", "composerepo://ticket/2025/02/04/test-ticket", emojiText(EmojiTicket) + "20250204testticket"},`                                                     |
| 106 | 10040 | `composerepo://goals`                                                             | `{"goals", "composerepo://goals", emojiText(EmojiGoal)},`                                                                                                       |
| 107 | 10041 | `composerepo://goal/RUNNING-SKETCHPAD`                                            | `{"goal", "composerepo://goal/RUNNING-SKETCHPAD", emojiText(EmojiGoal) + "runningsketchpad"},`                                                                  |
| 108 | 10042 | `composerepo://goal/R26-02/RUNNING-SKETCHPAD`                                     | `{"goal nested", "composerepo://goal/R26-02/RUNNING-SKETCHPAD", emojiText(EmojiGoal) + "r2602" + emojiText(EmojiGoal) + "runningsketchpad"},`                   |
| 109 | 10043 | `composerepo://drafts`                                                            | `{"drafts", "composerepo://drafts", emojiText(EmojiDraft)},`                                                                                                    |
| 110 | 10044 | `composerepo://draft/my-draft`                                                    | `{"draft", "composerepo://draft/my-draft", emojiText(EmojiDraft) + "mydraft"},`                                                                                 |
| 111 | 10045 | `composerepo://todos`                                                             | `{"todos", "composerepo://todos", emojiText(EmojiTodo)},`                                                                                                       |
| 112 | 10046 | `composerepo://todo/my-todo`                                                      | `{"todo", "composerepo://todo/my-todo", emojiText(EmojiTodo) + "mytodo"},`                                                                                      |
| 113 | 10047 | `composerepo://policies`                                                          | `{"policies", "composerepo://policies", emojiText(EmojiPolicy)},`                                                                                               |
| 114 | 10048 | `composerepo://policy/code-hygiene`                                               | `{"policy", "composerepo://policy/code-hygiene", emojiText(EmojiPolicy) + "codehygiene"},`                                                                      |
| 115 | 10049 | `composerepo://statutes`                                                          | `{"statutes", "composerepo://statutes", ""},`                                                                                                                   |
| 116 | 10050 | `composerepo://statute/code/inline-comment`                                       | `{"statute", "composerepo://statute/code/inline-comment", ""},`                                                                                                 |
| 117 | 10051 | `composerepo://contributors`                                                      | `{"contributors", "composerepo://contributors", emojiText(EmojiContributor)},`                                                                                  |
| 118 | 10052 | `composerepo://contributor/usalu`                                                 | `{"contributor", "composerepo://contributor/usalu", emojiText(EmojiContributor) + "usalu"},`                                                                    |
| 119 | 10053 | `composerepo://commits`                                                           | `{"commits", "composerepo://commits", emojiText(EmojiCommit)},`                                                                                                 |
| 120 | 10054 | `composerepo://commit/abc123`                                                     | `{"commit", "composerepo://commit/abc123", emojiText(EmojiCommit) + "abc123"},`                                                                                 |
| 121 | 10055 | `composerepo://interactions`                                                      | `{"interactions", "composerepo://interactions", ""},`                                                                                                           |
| 122 | 10056 | `composerepo://interaction/on/ticket/testticket/started`                          | `{"interaction ticket", "composerepo://interaction/on/ticket/testticket/started", emojiText(EmojiTicket) + "testticket" + emojiText(EmojiInteractionStarted)},` |
| 123 | 10057 | `composerepo://interaction/on/goal/r2602/started`                                 | `{"interaction goal", "composerepo://interaction/on/goal/r2602/started", emojiText(EmojiGoal) + "r2602" + emojiText(EmojiInteractionStarted)},`                 |

---

## 15. `TestIdUriRoundTrip` (line ~10290)

| #   | Line  | URI                                             | Code                                                                                                                                          |
| --- | ----- | ----------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| 124 | 10310 | `composerepo://policy/codehygiene`                | `{"policy", emojiText(EmojiPolicy) + "codehygiene", "composerepo://policy/codehygiene"},`                                                       |
| 125 | 10311 | `composerepo://contributor/usalu`                 | `{"contributor", emojiText(EmojiContributor) + "usalu", "composerepo://contributor/usalu"},`                                                    |
| 126 | 10312 | `composerepo://commit/abc123`                     | `{"commit", emojiText(EmojiCommit) + "abc123", "composerepo://commit/abc123"},`                                                                 |
| 127 | 10313 | `composerepo://draft/mydraft`                     | `{"draft", emojiText(EmojiDraft) + "mydraft", "composerepo://draft/mydraft"},`                                                                  |
| 128 | 10314 | `composerepo://section/imports`                   | `{"section", emojiText(EmojiSection) + "imports", "composerepo://section/imports"},`                                                            |
| 129 | 10315 | `composerepo://file/indexts`                      | `{"file", emojiText(EmojiFileCode) + "indexts", "composerepo://file/indexts"},`                                                                 |
| 130 | 10316 | `composerepo://ticket/20260115someticket`         | `{"ticket", emojiText(EmojiTicket) + "20260115someticket", "composerepo://ticket/20260115someticket"},`                                         |
| 131 | 10317 | `composerepo://goal/r2602running`                 | `{"goal", emojiText(EmojiGoal) + "r2602running", "composerepo://goal/r2602running"},`                                                           |
| 132 | 10318 | `composerepo://interaction/on/goal/r2602/started` | `{"interaction goal", emojiText(EmojiGoal) + "r2602" + emojiText(EmojiInteractionStarted), "composerepo://interaction/on/goal/r2602/started"},` |
| 133 | 10319 | `composerepo://project/compose`                     | `{"project", emojiText(EmojiProjectUser) + "compose", "composerepo://project/compose"},`                                                            |
| 134 | 10320 | `composerepo://bundle/compose/js`                   | `{"bundle", emojiText(EmojiProjectUser) + "compose" + emojiText(EmojiBundleLibrary) + "js", "composerepo://bundle/compose/js"},`                    |

---

## 16. `TestToolGoalUri` (line ~11170)

| #   | Line  | URI                 | Code                                                |
| --- | ----- | ------------------- | --------------------------------------------------- |
| 135 | 11178 | `composerepo://goal/` | `if !strings.HasPrefix(uri, "composerepo://goal/") {` |

---

## 17. `TestParityProjectTree` (line ~11380)

| #   | Line  | URI                    | Code                                                                   |
| --- | ----- | ---------------------- | ---------------------------------------------------------------------- |
| 136 | 11392 | `composerepo://project/` | `if idx := strings.Index(trimmed, "composerepo://project/"); idx >= 0 {` |
| 137 | 11393 | `composerepo://project/` | `nameStart := idx + len("composerepo://project/")`                       |

---

## 18. `TestRenderMonorepoTree` (line ~11960)

| #   | Line  | URI                    | Code                                                                                                                |
| --- | ----- | ---------------------- | ------------------------------------------------------------------------------------------------------------------- |
| 138 | 11976 | `composerepo://projects` | `{Kind: TreeNodeCategory, ID: "projects", Label: "🏗️Projects", URI: "composerepo://projects", Children: []*TreeNode{` |
| 139 | 11993 | `composerepo://goals`    | `{Kind: TreeNodeCategory, ID: "goals", Label: "🎯Goals", URI: "composerepo://goals"},`                                |
| 140 | 12028 | `composerepo://projects` | `{Kind: TreeNodeCategory, ID: "projects", Label: "🏗️Projects", URI: "composerepo://projects", Children: []*TreeNode{` |
| 141 | 12065 | `composerepo://goals`    | `{Kind: TreeNodeCategory, ID: "goals", Label: "🎯Goals", URI: "composerepo://goals", Children: []*TreeNode{`          |

---

## 19. `TestUnifiedRenderingGoalIdentity` (line ~12570)

| #   | Line  | URI                          | Code                                   |
| --- | ----- | ---------------------------- | -------------------------------------- |
| 142 | 12583 | `composerepo://goal/test-goal` | `URI:   "composerepo://goal/test-goal",` |
| 143 | 12609 | `composerepo://goal/test-goal` | `URI:   "composerepo://goal/test-goal",` |

---

## 20. `TestUnifiedRenderingSectionIdentity` (line ~12800)

| #   | Line  | URI                                          | Code                                                   |
| --- | ----- | -------------------------------------------- | ------------------------------------------------------ |
| 144 | 12815 | `composerepo://section/test/file.ts/mysection` | `URI:   "composerepo://section/test/file.ts/mysection",` |

---

## Summary by URI Resource Kind

| Resource Kind            | Count | Functions                                                                                                                                                      |
| ------------------------ | ----- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `root`                   | 2     | TestArtifactIDAndURI, TestUriToId                                                                                                                              |
| `projects`               | 3     | TestArtifactIDAndURI, TestUriToId, TestRenderMonorepoTree(×2)                                                                                                  |
| `project/`               | 9     | TestArtifactIDAndURI(×3), TestIdToUri(×2), TestUriToId(×2), TestIdUriRoundTrip, TestParityProjectTree(×2)                                                      |
| `bundles`                | 2     | TestArtifactIDAndURI, TestUriToId                                                                                                                              |
| `bundle/`                | 6     | TestArtifactIDAndURI(×3), TestIdToUri, TestUriToId, TestIdUriRoundTrip                                                                                         |
| `folders`                | 3     | TestArtifactIDAndURI(×2), TestUriToId(×2)                                                                                                                      |
| `folder/`                | 5     | TestArtifactIDAndURI(×2), TestIdToUri(×2), TestUriToId                                                                                                         |
| `files`                  | 2     | TestArtifactIDAndURI, TestUriToId                                                                                                                              |
| `file/`                  | 14    | TestFormatHeader*(×4), TestCliE2E*, TestArtifactIDAndURI(×7), TestIdToUri(×2), TestUriToId, TestIdUriRoundTrip                                                 |
| `sections`               | 3     | TestArtifactIDAndURI, TestIdToUri, TestUriToId                                                                                                                 |
| `section/`               | 9     | TestSectionIdentificationAutofix, TestArtifactIDAndURI(×2), TestIdToUri, TestUriToId, TestIdUriRoundTrip, TestUnifiedRenderingSectionIdentity                  |
| `definitions`            | 2     | TestArtifactIDAndURI, TestUriToId                                                                                                                              |
| `definition/`            | 12    | TestDefinitionIdentificationAutofix, TestDefinitionNativeDocstringAutofix, TestPythonTripleQuote\*(×2), TestArtifactIDAndURI(×4), TestIdToUri, TestUriToId(×2) |
| `tickets`                | 3     | TestArtifactIDAndURI, TestIdToUri, TestUriToId                                                                                                                 |
| `ticket/`                | 5     | TestArtifactIDAndURI(×2), TestIdToUri, TestUriToId, TestIdUriRoundTrip                                                                                         |
| `goals`                  | 4     | TestArtifactIDAndURI, TestIdToUri, TestUriToId, TestRenderMonorepoTree(×2)                                                                                     |
| `goal/`                  | 10    | TestArtifactIDAndURI(×2), TestIdToUri(×2), TestUriToId(×2), TestIdUriRoundTrip, TestToolGoalUri, TestUnifiedRenderingGoalIdentity(×2)                          |
| `drafts`                 | 2     | TestArtifactIDAndURI, TestUriToId                                                                                                                              |
| `draft/`                 | 3     | TestArtifactIDAndURI, TestIdToUri, TestUriToId, TestIdUriRoundTrip                                                                                             |
| `todos`                  | 2     | TestArtifactIDAndURI, TestUriToId                                                                                                                              |
| `todo/`                  | 2     | TestArtifactIDAndURI, TestUriToId                                                                                                                              |
| `policies`               | 3     | TestArtifactIDAndURI, TestIdToUri, TestUriToId                                                                                                                 |
| `policy/`                | 4     | TestArtifactIDAndURI, TestIdToUri, TestUriToId, TestIdUriRoundTrip                                                                                             |
| `statutes`               | 2     | TestArtifactIDAndURI, TestUriToId                                                                                                                              |
| `statute/`               | 2     | TestArtifactIDAndURI, TestUriToId                                                                                                                              |
| `contributors`           | 3     | TestArtifactIDAndURI, TestIdToUri, TestUriToId                                                                                                                 |
| `contributor/`           | 4     | TestArtifactIDAndURI, TestIdToUri, TestUriToId, TestIdUriRoundTrip                                                                                             |
| `commits`                | 3     | TestArtifactIDAndURI, TestIdToUri, TestUriToId                                                                                                                 |
| `commit/`                | 4     | TestArtifactIDAndURI, TestIdToUri, TestUriToId, TestIdUriRoundTrip                                                                                             |
| `interactions`           | 2     | TestArtifactIDAndURI, TestUriToId                                                                                                                              |
| `interaction/`           | 5     | TestArtifactIDAndURI(×2), TestIdToUri(×2), TestUriToId(×2), TestIdUriRoundTrip                                                                                 |
| `territory/`             | 1     | TestTerritory                                                                                                                                                  |
| (generic `composerepo://`) | 1     | TestRenderEntityMarkdownLink_AllKinds                                                                                                                          |
