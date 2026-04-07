# All Hardcoded `"semiorepo://` URI Strings in `main_test.go`

Total: **144 occurrences** across **18 test functions**

---

## 1. `TestSectionIdentificationAutofix` (line 3474)

| #   | Line | URI                    | Code                                                           |
| --- | ---- | ---------------------- | -------------------------------------------------------------- |
| 1   | 3510 | `semiorepo://section/` | `if !strings.Contains(fixedContent, "semiorepo://section/") {` |

---

## 2. `TestDefinitionIdentificationAutofix` (line 3515)

| #   | Line | URI                       | Code                                                              |
| --- | ---- | ------------------------- | ----------------------------------------------------------------- |
| 2   | 3551 | `semiorepo://definition/` | `if !strings.Contains(fixedContent, "semiorepo://definition/") {` |

---

## 3. `TestDefinitionNativeDocstringAutofix` (line 3848)

| #   | Line | URI                       | Code                                                              |
| --- | ---- | ------------------------- | ----------------------------------------------------------------- |
| 3   | 3896 | `semiorepo://definition/` | `if !strings.Contains(fixedContent, "semiorepo://definition/") {` |

---

## 4. `TestPythonTripleQuoteDocstringAutofix` (line 3901)

| #   | Line | URI                       | Code                                                              |
| --- | ---- | ------------------------- | ----------------------------------------------------------------- |
| 4   | 3943 | `semiorepo://definition/` | `if !strings.Contains(fixedContent, "semiorepo://definition/") {` |

---

## 5. `TestPythonTripleQuoteDocstringMerge` (line 3954)

| #   | Line | URI                       | Code                                                              |
| --- | ---- | ------------------------- | ----------------------------------------------------------------- |
| 5   | 3996 | `semiorepo://definition/` | `if !strings.Contains(fixedContent, "semiorepo://definition/") {` |

---

## 6. `TestFormatHeaderStructure` (line 4493)

| #   | Line | URI                             | Code                                                                                                                                                                             |
| --- | ---- | ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 6   | 4495 | `semiorepo://file/test/file.ts` | `header := lang.FormatHeader("💻test/file.ts", "semiorepo://file/test/file.ts", "A test file", "2025 Test User <test@test.com>", "AGPL license text here", "Some requirements")` |

---

## 7. `TestFormatHeaderEmptyRequirements` (line 4519)

| #   | Line | URI                             | Code                                                                                                                            |
| --- | ---- | ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| 7   | 4521 | `semiorepo://file/test/file.go` | `header := lang.FormatHeader("💻test/file.go", "semiorepo://file/test/file.go", "", "2025 Dev <dev@dev.com>", "AGPL text", "")` |

---

## 8. `TestFormatHeaderAllLanguages` (line 4530)

| #   | Line | URI                          | Code                                                                                                             |
| --- | ---- | ---------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| 8   | 4543 | `semiorepo://file/test/file` | `header := lang.FormatHeader("💻test/file", "semiorepo://file/test/file", "", "2025 Dev <d@d.com>", "AGPL", "")` |
| 9   | 4557 | `semiorepo://file/test/file` | `header := lang.FormatHeader("💻test/file", "semiorepo://file/test/file", "", "2025 Dev <d@d.com>", "AGPL", "")` |

---

## 9. `TestTerritory` (line 4593)

| #   | Line | URI                      | Code                                                     |
| --- | ---- | ------------------------ | -------------------------------------------------------- |
| 10  | 4716 | `semiorepo://territory/` | `if !strings.HasPrefix(uri, "semiorepo://territory/") {` |

---

## 10. `TestCliE2E_TicketLifecycle_Syntaxes_NoManagement` (line 7310)

| #   | Line | URI                 | Code                                                      |
| --- | ---- | ------------------- | --------------------------------------------------------- |
| 11  | 7350 | `semiorepo://file/` | `fileURI := "semiorepo://file/" + PathToUriPath(fileRel)` |

---

## 11. `TestRenderEntityMarkdownLink_AllKinds` (line ~8740)

| #   | Line | URI            | Code                                             |
| --- | ---- | -------------- | ------------------------------------------------ |
| 12  | 8763 | `semiorepo://` | `if !strings.Contains(output, "semiorepo://") {` |

---

## 12. `TestArtifactIDAndURI` (line ~9580)

| #   | Line | URI                                                                                                | Code                                                                                                           |
| --- | ---- | -------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| 13  | 9601 | `semiorepo://root`                                                                                 | `wantURI: "semiorepo://root",`                                                                                 |
| 14  | 9608 | `semiorepo://projects`                                                                             | `wantURI: "semiorepo://projects",`                                                                             |
| 15  | 9615 | `semiorepo://project/semio`                                                                        | `wantURI: "semiorepo://project/semio",`                                                                        |
| 16  | 9622 | `semiorepo://project/repo`                                                                         | `wantURI: "semiorepo://project/repo",`                                                                         |
| 17  | 9629 | `semiorepo://project/coda`                                                                         | `wantURI: "semiorepo://project/coda",`                                                                         |
| 18  | 9636 | `semiorepo://bundles`                                                                              | `wantURI: "semiorepo://bundles",`                                                                              |
| 19  | 9643 | `semiorepo://bundle/semio/js`                                                                      | `wantURI: "semiorepo://bundle/semio/js",`                                                                      |
| 20  | 9650 | `semiorepo://bundle/coda/examples`                                                                 | `wantURI: "semiorepo://bundle/coda/examples",`                                                                 |
| 21  | 9657 | `semiorepo://bundle/semio/desktop`                                                                 | `wantURI: "semiorepo://bundle/semio/desktop",`                                                                 |
| 22  | 9664 | `semiorepo://folders`                                                                              | `wantURI: "semiorepo://folders",`                                                                              |
| 23  | 9671 | `semiorepo://folders/semio/js/src`                                                                 | `wantURI: "semiorepo://folders/semio/js/src",`                                                                 |
| 24  | 9678 | `semiorepo://folder/semio/js/src`                                                                  | `wantURI: "semiorepo://folder/semio/js/src",`                                                                  |
| 25  | 9685 | `semiorepo://folder/semio/js/utils`                                                                | `wantURI: "semiorepo://folder/semio/js/utils",`                                                                |
| 26  | 9692 | `semiorepo://files`                                                                                | `wantURI: "semiorepo://files",`                                                                                |
| 27  | 9699 | `semiorepo://file/test.txt`                                                                        | `wantURI: "semiorepo://file/test.txt",`                                                                        |
| 28  | 9706 | `semiorepo://file/main.go`                                                                         | `wantURI: "semiorepo://file/main.go",`                                                                         |
| 29  | 9713 | `semiorepo://file/semio/js/src/index.test.ts`                                                      | `wantURI: "semiorepo://file/semio/js/src/index.test.ts",`                                                      |
| 30  | 9720 | `semiorepo://file/tsconfig.json`                                                                   | `wantURI: "semiorepo://file/tsconfig.json",`                                                                   |
| 31  | 9727 | `semiorepo://file/build.sh`                                                                        | `wantURI: "semiorepo://file/build.sh",`                                                                        |
| 32  | 9734 | `semiorepo://file/logo.png`                                                                        | `wantURI: "semiorepo://file/logo.png",`                                                                        |
| 33  | 9741 | `semiorepo://file/LICENSE.md`                                                                      | `wantURI: "semiorepo://file/LICENSE.md",`                                                                      |
| 34  | 9748 | `semiorepo://sections/semio/js/src/index.ts`                                                       | `wantURI: "semiorepo://sections/semio/js/src/index.ts",`                                                       |
| 35  | 9755 | `semiorepo://section/semio/js/src/Design.tsx/State%20Management/Design%20Store`                    | `wantURI: "semiorepo://section/semio/js/src/Design.tsx/State%20Management/Design%20Store",`                    |
| 36  | 9762 | `semiorepo://section/semio/js/src/file.ts/Imports`                                                 | `wantURI: "semiorepo://section/semio/js/src/file.ts/Imports",`                                                 |
| 37  | 9769 | `semiorepo://definitions/semio/js/src/index.ts`                                                    | `wantURI: "semiorepo://definitions/semio/js/src/index.ts",`                                                    |
| 38  | 9776 | `semiorepo://definition/semio/js/src/index.ts/MyClass`                                             | `wantURI: "semiorepo://definition/semio/js/src/index.ts/MyClass",`                                             |
| 39  | 9783 | `semiorepo://definition/semio/js/src/file.ts/Types/MyInterface`                                    | `wantURI: "semiorepo://definition/semio/js/src/file.ts/Types/MyInterface",`                                    |
| 40  | 9790 | `semiorepo://definition/repo/cli/main.go/GraphQL%20Types/GraphQL%20Input%20Types/TicketCloseInput` | `wantURI: "semiorepo://definition/repo/cli/main.go/GraphQL%20Types/GraphQL%20Input%20Types/TicketCloseInput",` |
| 41  | 9797 | `semiorepo://definition/semio/js/src/file.ts/MAX_SIZE`                                             | `wantURI: "semiorepo://definition/semio/js/src/file.ts/MAX_SIZE",`                                             |
| 42  | 9804 | `semiorepo://tickets`                                                                              | `wantURI: "semiorepo://tickets",`                                                                              |
| 43  | 9816 | `semiorepo://ticket/2025/02/04/test-ticket`                                                        | `wantURI: "semiorepo://ticket/2025/02/04/test-ticket",`                                                        |
| 44  | 9829 | `semiorepo://ticket/2025/02/04/test-ticket`                                                        | `wantURI: "semiorepo://ticket/2025/02/04/test-ticket",`                                                        |
| 45  | 9836 | `semiorepo://goals`                                                                                | `wantURI: "semiorepo://goals",`                                                                                |
| 46  | 9843 | `semiorepo://goal/RUNNING-SKETCHPAD`                                                               | `wantURI: "semiorepo://goal/RUNNING-SKETCHPAD",`                                                               |
| 47  | 9850 | `semiorepo://goal/R26-02/RUNNING-SKETCHPAD`                                                        | `wantURI: "semiorepo://goal/R26-02/RUNNING-SKETCHPAD",`                                                        |
| 48  | 9857 | `semiorepo://drafts`                                                                               | `wantURI: "semiorepo://drafts",`                                                                               |
| 49  | 9864 | `semiorepo://draft/my-draft`                                                                       | `wantURI: "semiorepo://draft/my-draft",`                                                                       |
| 50  | 9871 | `semiorepo://todos`                                                                                | `wantURI: "semiorepo://todos",`                                                                                |
| 51  | 9878 | `semiorepo://todo/my-todo`                                                                         | `wantURI: "semiorepo://todo/my-todo",`                                                                         |
| 52  | 9885 | `semiorepo://policies`                                                                             | `wantURI: "semiorepo://policies",`                                                                             |
| 53  | 9892 | `semiorepo://policy/code-hygiene`                                                                  | `wantURI: "semiorepo://policy/code-hygiene",`                                                                  |
| 54  | 9899 | `semiorepo://statutes`                                                                             | `wantURI: "semiorepo://statutes",`                                                                             |
| 55  | 9906 | `semiorepo://statute/code/inline-comment`                                                          | `wantURI: "semiorepo://statute/code/inline-comment",`                                                          |
| 56  | 9913 | `semiorepo://contributors`                                                                         | `wantURI: "semiorepo://contributors",`                                                                         |
| 57  | 9920 | `semiorepo://contributor/usalu`                                                                    | `wantURI: "semiorepo://contributor/usalu",`                                                                    |
| 58  | 9927 | `semiorepo://commits`                                                                              | `wantURI: "semiorepo://commits",`                                                                              |
| 59  | 9934 | `semiorepo://commit/abc123`                                                                        | `wantURI: "semiorepo://commit/abc123",`                                                                        |
| 60  | 9941 | `semiorepo://interactions`                                                                         | `wantURI: "semiorepo://interactions",`                                                                         |
| 61  | 9948 | `semiorepo://interaction/on/ticket/introduceinteractionmechanism/started`                          | `wantURI: "semiorepo://interaction/on/ticket/introduceinteractionmechanism/started",`                          |
| 62  | 9955 | `semiorepo://interaction/on/goal/r2602/finished`                                                   | `wantURI: "semiorepo://interaction/on/goal/r2602/finished",`                                                   |

---

## 13. `TestIdToUri` (line ~9965)

| #   | Line  | URI                                                               | Code                                                                                                                                      |
| --- | ----- | ----------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| 63  | 9979  | `semiorepo://project/semio`                                       | `{"project user", emojiText(EmojiProjectUser) + "semio", "semiorepo://project/semio"},`                                                   |
| 64  | 9980  | `semiorepo://project/semiorepo`                                   | `{"project infra", emojiText(EmojiProjectInfra) + "semiorepo", "semiorepo://project/semiorepo"},`                                         |
| 65  | 9981  | `semiorepo://bundle/semio/js`                                     | `{"bundle", emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js", "semiorepo://bundle/semio/js"},`                |
| 66  | 9982  | `semiorepo://folder/src`                                          | `{"folder required", emojiText(EmojiFolderRequired) + "src", "semiorepo://folder/src"},`                                                  |
| 67  | 9983  | `semiorepo://folder/utils`                                        | `{"folder org", emojiText(EmojiFolderOrg) + "utils", "semiorepo://folder/utils"},`                                                        |
| 68  | 9984  | `semiorepo://file/testtxt`                                        | `{"file docs", emojiText(EmojiFileDocs) + "testtxt", "semiorepo://file/testtxt"},`                                                        |
| 69  | 9985  | `semiorepo://file/maingo`                                         | `{"file code", emojiText(EmojiFileCode) + "maingo", "semiorepo://file/maingo"},`                                                          |
| 70  | 9986  | `semiorepo://sections`                                            | `{"section collection", emojiText(EmojiSection), "semiorepo://sections"},`                                                                |
| 71  | 9987  | `semiorepo://section/semio/js/src/designtsx/statemanagment/store` | `{"section", buildSectionID(...), "semiorepo://section/semio/js/src/designtsx/statemanagment/store"},`                                    |
| 72  | 9988  | `semiorepo://definition/semio/js/src/filets/types/myclass`        | `{"definition impl", buildDefinitionID(...), "semiorepo://definition/semio/js/src/filets/types/myclass"},`                                |
| 73  | 9989  | `semiorepo://tickets`                                             | `{"ticket collection", emojiText(EmojiTicket), "semiorepo://tickets"},`                                                                   |
| 74  | 9990  | `semiorepo://ticket/testticket`                                   | `{"ticket", emojiText(EmojiTicket) + "testticket", "semiorepo://ticket/testticket"},`                                                     |
| 75  | 9991  | `semiorepo://goals`                                               | `{"goal collection", emojiText(EmojiGoal), "semiorepo://goals"},`                                                                         |
| 76  | 9992  | `semiorepo://goal/r2602runningsketchpad`                          | `{"goal", emojiText(EmojiGoal) + "r2602runningsketchpad", "semiorepo://goal/r2602runningsketchpad"},`                                     |
| 77  | 9993  | `semiorepo://goal/r2602/runningsketchpad`                         | `{"goal nested", emojiText(EmojiGoal) + "r2602" + emojiText(EmojiGoal) + "runningsketchpad", "semiorepo://goal/r2602/runningsketchpad"},` |
| 78  | 9994  | `semiorepo://drafts`                                              | `{"draft collection", emojiText(EmojiDraft), "semiorepo://drafts"},`                                                                      |
| 79  | 9995  | `semiorepo://draft/mydraft`                                       | `{"draft", emojiText(EmojiDraft) + "mydraft", "semiorepo://draft/mydraft"},`                                                              |
| 80  | 9996  | `semiorepo://policies`                                            | `{"policy collection", emojiText(EmojiPolicy), "semiorepo://policies"},`                                                                  |
| 81  | 9997  | `semiorepo://policy/codehygiene`                                  | `{"policy", emojiText(EmojiPolicy) + "codehygiene", "semiorepo://policy/codehygiene"},`                                                   |
| 82  | 9998  | `semiorepo://contributors`                                        | `{"contributor collection", emojiText(EmojiContributor), "semiorepo://contributors"},`                                                    |
| 83  | 9999  | `semiorepo://contributor/usalu`                                   | `{"contributor", emojiText(EmojiContributor) + "usalu", "semiorepo://contributor/usalu"},`                                                |
| 84  | 10000 | `semiorepo://commits`                                             | `{"commit collection", emojiText(EmojiCommit), "semiorepo://commits"},`                                                                   |
| 85  | 10001 | `semiorepo://commit/abc123`                                       | `{"commit", emojiText(EmojiCommit) + "abc123", "semiorepo://commit/abc123"},`                                                             |
| 86  | 10002 | `semiorepo://interaction/on/ticket/testticket/started`            | `{"interaction started ticket", ..., "semiorepo://interaction/on/ticket/testticket/started"},`                                            |
| 87  | 10003 | `semiorepo://interaction/on/goal/r2602/finished`                  | `{"interaction finished goal", ..., "semiorepo://interaction/on/goal/r2602/finished"},`                                                   |

---

## 14. `TestUriToId` (line ~10010)

| #   | Line  | URI                                                                             | Code                                                                                                                                                          |
| --- | ----- | ------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 88  | 10022 | `semiorepo://root`                                                              | `{"repo", "semiorepo://root", ""},`                                                                                                                           |
| 89  | 10023 | `semiorepo://projects`                                                          | `{"projects", "semiorepo://projects", emojiText(EmojiProjects)},`                                                                                             |
| 90  | 10024 | `semiorepo://project/semio`                                                     | `{"project", "semiorepo://project/semio", emojiText(EmojiProjectUser) + "semio"},`                                                                            |
| 91  | 10025 | `semiorepo://project/repo`                                                      | `{"project infra", "semiorepo://project/repo", emojiText(EmojiProjectInfra) + "semiorepo"},`                                                                  |
| 92  | 10026 | `semiorepo://bundles`                                                           | `{"bundles", "semiorepo://bundles", emojiText(EmojiBundles)},`                                                                                                |
| 93  | 10027 | `semiorepo://bundle/semio/js`                                                   | `{"bundle", "semiorepo://bundle/semio/js", emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js"},`                                    |
| 94  | 10028 | `semiorepo://folders`                                                           | `{"folders", "semiorepo://folders", emojiText(EmojiFolders)},`                                                                                                |
| 95  | 10029 | `semiorepo://folders/semio/js/src`                                              | `{"folders with parent", "semiorepo://folders/semio/js/src", emojiText(EmojiFolders)},`                                                                       |
| 96  | 10030 | `semiorepo://folder/semio/js/src`                                               | `{"folder", "semiorepo://folder/semio/js/src", emojiText(EmojiFolderOrg) + "semiojssrc"},`                                                                    |
| 97  | 10031 | `semiorepo://files`                                                             | `{"files", "semiorepo://files", emojiText(EmojiFiles)},`                                                                                                      |
| 98  | 10032 | `semiorepo://file/test.txt`                                                     | `{"file", "semiorepo://file/test.txt", emojiText(EmojiFileCode) + "testtxt"},`                                                                                |
| 99  | 10033 | `semiorepo://sections`                                                          | `{"sections", "semiorepo://sections", emojiText(EmojiSections)},`                                                                                             |
| 100 | 10034 | `semiorepo://section/semio/js/src/Design.tsx/State%20Management/Design%20Store` | `{"section", "semiorepo://section/semio/js/src/Design.tsx/State%20Management/Design%20Store", buildSectionID(...)},`                                          |
| 101 | 10035 | `semiorepo://definitions`                                                       | `{"definitions", "semiorepo://definitions", emojiText(EmojiDefinitions)},`                                                                                    |
| 102 | 10036 | `semiorepo://definition/semio/js/src/file.ts/myFunc`                            | `{"definition single", "semiorepo://definition/semio/js/src/file.ts/myFunc", buildDefinitionID(...)},`                                                        |
| 103 | 10037 | `semiorepo://definition/semio/js/src/file.ts/Section/myFunc`                    | `{"definition with section", "semiorepo://definition/semio/js/src/file.ts/Section/myFunc", buildDefinitionID(...)},`                                          |
| 104 | 10038 | `semiorepo://tickets`                                                           | `{"tickets", "semiorepo://tickets", emojiText(EmojiTicket)},`                                                                                                 |
| 105 | 10039 | `semiorepo://ticket/2025/02/04/test-ticket`                                     | `{"ticket", "semiorepo://ticket/2025/02/04/test-ticket", emojiText(EmojiTicket) + "20250204testticket"},`                                                     |
| 106 | 10040 | `semiorepo://goals`                                                             | `{"goals", "semiorepo://goals", emojiText(EmojiGoal)},`                                                                                                       |
| 107 | 10041 | `semiorepo://goal/RUNNING-SKETCHPAD`                                            | `{"goal", "semiorepo://goal/RUNNING-SKETCHPAD", emojiText(EmojiGoal) + "runningsketchpad"},`                                                                  |
| 108 | 10042 | `semiorepo://goal/R26-02/RUNNING-SKETCHPAD`                                     | `{"goal nested", "semiorepo://goal/R26-02/RUNNING-SKETCHPAD", emojiText(EmojiGoal) + "r2602" + emojiText(EmojiGoal) + "runningsketchpad"},`                   |
| 109 | 10043 | `semiorepo://drafts`                                                            | `{"drafts", "semiorepo://drafts", emojiText(EmojiDraft)},`                                                                                                    |
| 110 | 10044 | `semiorepo://draft/my-draft`                                                    | `{"draft", "semiorepo://draft/my-draft", emojiText(EmojiDraft) + "mydraft"},`                                                                                 |
| 111 | 10045 | `semiorepo://todos`                                                             | `{"todos", "semiorepo://todos", emojiText(EmojiTodo)},`                                                                                                       |
| 112 | 10046 | `semiorepo://todo/my-todo`                                                      | `{"todo", "semiorepo://todo/my-todo", emojiText(EmojiTodo) + "mytodo"},`                                                                                      |
| 113 | 10047 | `semiorepo://policies`                                                          | `{"policies", "semiorepo://policies", emojiText(EmojiPolicy)},`                                                                                               |
| 114 | 10048 | `semiorepo://policy/code-hygiene`                                               | `{"policy", "semiorepo://policy/code-hygiene", emojiText(EmojiPolicy) + "codehygiene"},`                                                                      |
| 115 | 10049 | `semiorepo://statutes`                                                          | `{"statutes", "semiorepo://statutes", ""},`                                                                                                                   |
| 116 | 10050 | `semiorepo://statute/code/inline-comment`                                       | `{"statute", "semiorepo://statute/code/inline-comment", ""},`                                                                                                 |
| 117 | 10051 | `semiorepo://contributors`                                                      | `{"contributors", "semiorepo://contributors", emojiText(EmojiContributor)},`                                                                                  |
| 118 | 10052 | `semiorepo://contributor/usalu`                                                 | `{"contributor", "semiorepo://contributor/usalu", emojiText(EmojiContributor) + "usalu"},`                                                                    |
| 119 | 10053 | `semiorepo://commits`                                                           | `{"commits", "semiorepo://commits", emojiText(EmojiCommit)},`                                                                                                 |
| 120 | 10054 | `semiorepo://commit/abc123`                                                     | `{"commit", "semiorepo://commit/abc123", emojiText(EmojiCommit) + "abc123"},`                                                                                 |
| 121 | 10055 | `semiorepo://interactions`                                                      | `{"interactions", "semiorepo://interactions", ""},`                                                                                                           |
| 122 | 10056 | `semiorepo://interaction/on/ticket/testticket/started`                          | `{"interaction ticket", "semiorepo://interaction/on/ticket/testticket/started", emojiText(EmojiTicket) + "testticket" + emojiText(EmojiInteractionStarted)},` |
| 123 | 10057 | `semiorepo://interaction/on/goal/r2602/started`                                 | `{"interaction goal", "semiorepo://interaction/on/goal/r2602/started", emojiText(EmojiGoal) + "r2602" + emojiText(EmojiInteractionStarted)},`                 |

---

## 15. `TestIdUriRoundTrip` (line ~10290)

| #   | Line  | URI                                             | Code                                                                                                                                          |
| --- | ----- | ----------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| 124 | 10310 | `semiorepo://policy/codehygiene`                | `{"policy", emojiText(EmojiPolicy) + "codehygiene", "semiorepo://policy/codehygiene"},`                                                       |
| 125 | 10311 | `semiorepo://contributor/usalu`                 | `{"contributor", emojiText(EmojiContributor) + "usalu", "semiorepo://contributor/usalu"},`                                                    |
| 126 | 10312 | `semiorepo://commit/abc123`                     | `{"commit", emojiText(EmojiCommit) + "abc123", "semiorepo://commit/abc123"},`                                                                 |
| 127 | 10313 | `semiorepo://draft/mydraft`                     | `{"draft", emojiText(EmojiDraft) + "mydraft", "semiorepo://draft/mydraft"},`                                                                  |
| 128 | 10314 | `semiorepo://section/imports`                   | `{"section", emojiText(EmojiSection) + "imports", "semiorepo://section/imports"},`                                                            |
| 129 | 10315 | `semiorepo://file/indexts`                      | `{"file", emojiText(EmojiFileCode) + "indexts", "semiorepo://file/indexts"},`                                                                 |
| 130 | 10316 | `semiorepo://ticket/20260115someticket`         | `{"ticket", emojiText(EmojiTicket) + "20260115someticket", "semiorepo://ticket/20260115someticket"},`                                         |
| 131 | 10317 | `semiorepo://goal/r2602running`                 | `{"goal", emojiText(EmojiGoal) + "r2602running", "semiorepo://goal/r2602running"},`                                                           |
| 132 | 10318 | `semiorepo://interaction/on/goal/r2602/started` | `{"interaction goal", emojiText(EmojiGoal) + "r2602" + emojiText(EmojiInteractionStarted), "semiorepo://interaction/on/goal/r2602/started"},` |
| 133 | 10319 | `semiorepo://project/semio`                     | `{"project", emojiText(EmojiProjectUser) + "semio", "semiorepo://project/semio"},`                                                            |
| 134 | 10320 | `semiorepo://bundle/semio/js`                   | `{"bundle", emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js", "semiorepo://bundle/semio/js"},`                    |

---

## 16. `TestToolGoalUri` (line ~11170)

| #   | Line  | URI                 | Code                                                |
| --- | ----- | ------------------- | --------------------------------------------------- |
| 135 | 11178 | `semiorepo://goal/` | `if !strings.HasPrefix(uri, "semiorepo://goal/") {` |

---

## 17. `TestParityProjectTree` (line ~11380)

| #   | Line  | URI                    | Code                                                                   |
| --- | ----- | ---------------------- | ---------------------------------------------------------------------- |
| 136 | 11392 | `semiorepo://project/` | `if idx := strings.Index(trimmed, "semiorepo://project/"); idx >= 0 {` |
| 137 | 11393 | `semiorepo://project/` | `nameStart := idx + len("semiorepo://project/")`                       |

---

## 18. `TestRenderMonorepoTree` (line ~11960)

| #   | Line  | URI                    | Code                                                                                                                |
| --- | ----- | ---------------------- | ------------------------------------------------------------------------------------------------------------------- |
| 138 | 11976 | `semiorepo://projects` | `{Kind: TreeNodeCategory, ID: "projects", Label: "🏗️Projects", URI: "semiorepo://projects", Children: []*TreeNode{` |
| 139 | 11993 | `semiorepo://goals`    | `{Kind: TreeNodeCategory, ID: "goals", Label: "🎯Goals", URI: "semiorepo://goals"},`                                |
| 140 | 12028 | `semiorepo://projects` | `{Kind: TreeNodeCategory, ID: "projects", Label: "🏗️Projects", URI: "semiorepo://projects", Children: []*TreeNode{` |
| 141 | 12065 | `semiorepo://goals`    | `{Kind: TreeNodeCategory, ID: "goals", Label: "🎯Goals", URI: "semiorepo://goals", Children: []*TreeNode{`          |

---

## 19. `TestUnifiedRenderingGoalIdentity` (line ~12570)

| #   | Line  | URI                          | Code                                   |
| --- | ----- | ---------------------------- | -------------------------------------- |
| 142 | 12583 | `semiorepo://goal/test-goal` | `URI:   "semiorepo://goal/test-goal",` |
| 143 | 12609 | `semiorepo://goal/test-goal` | `URI:   "semiorepo://goal/test-goal",` |

---

## 20. `TestUnifiedRenderingSectionIdentity` (line ~12800)

| #   | Line  | URI                                          | Code                                                   |
| --- | ----- | -------------------------------------------- | ------------------------------------------------------ |
| 144 | 12815 | `semiorepo://section/test/file.ts/mysection` | `URI:   "semiorepo://section/test/file.ts/mysection",` |

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
| (generic `semiorepo://`) | 1     | TestRenderEntityMarkdownLink_AllKinds                                                                                                                          |
