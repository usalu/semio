
func TestEntityKinds(t *testing.T) {
expected := []string{
"root", "year", "month", "day", "hour", "minute", "second",
"project", "bundle", "folder", "file", "line", "range",
"section", "definition", "goal", "ticket", "draft", "todo",
"policy", "breach", "contributor", "commit", "interaction",
}
if len(EntityKinds) != len(expected) {
t.Fatalf("EntityKinds length: expected %d, got %d", len(expected), len(EntityKinds))
}
for i, e := range expected {
if EntityKinds[i] != e {
t.Errorf("EntityKinds[%d]: expected %q, got %q", i, e, EntityKinds[i])
}
}
}

func TestResourceKinds(t *testing.T) {
expected := []string{"repo", "project", "bundle", "folder", "file", "section", "definition"}
if len(ResourceKinds) != len(expected) {
t.Fatalf("ResourceKinds length: expected %d, got %d", len(expected), len(ResourceKinds))
}
for i, e := range expected {
if ResourceKinds[i] != e {
t.Errorf("ResourceKinds[%d]: expected %q, got %q", i, e, ResourceKinds[i])
}
}
}

func TestDiffableKinds(t *testing.T) {
expected := []string{
"root", "year", "month", "day", "hour",
"project", "bundle", "folder", "file", "section", "definition",
"goal", "ticket", "contributor", "commit", "interaction",
}
if len(DiffableKinds) != len(expected) {
t.Fatalf("DiffableKinds length: expected %d, got %d", len(expected), len(DiffableKinds))
}
for i, e := range expected {
if DiffableKinds[i] != e {
t.Errorf("DiffableKinds[%d]: expected %q, got %q", i, e, DiffableKinds[i])
}
}
}

func TestRelatedToFileKinds(t *testing.T) {
expected := []string{
"root", "year", "month", "day", "hour", "minute", "second",
"project", "bundle", "folder", "goal", "ticket", "draft", "todo",
"policy", "breach", "contributor", "commit", "interaction",
}
if len(RelatedToFileKinds) != len(expected) {
t.Fatalf("RelatedToFileKinds length: expected %d, got %d", len(expected), len(RelatedToFileKinds))
}
for i, e := range expected {
if RelatedToFileKinds[i] != e {
t.Errorf("RelatedToFileKinds[%d]: expected %q, got %q", i, e, RelatedToFileKinds[i])
}
}
}

func TestProjectListIDs(t *testing.T) {
result := ToolProjectList()
if result.Error != "" {
t.Fatalf("ToolProjectList returned error: %s", result.Error)
}
projects, ok := result.Data.([]Project)
if !ok {
t.Fatal("ToolProjectList data is not []Project")
}
expectedIDs := map[string]string{
"semio":      emojiText(EmojiProjectUser) + "semio",
"semio-repo": emojiText(EmojiProjectInfra) + "semiorepo",
"coda":       emojiText(EmojiProjectResearch) + "coda",
}
for _, p := range projects {
expected, ok := expectedIDs[p.Name]
if !ok {
continue
}
got := p.GetID()
if got != expected {
t.Errorf("project %q id: expected %q, got %q", p.Name, expected, got)
}
delete(expectedIDs, p.Name)
}
for name := range expectedIDs {
t.Errorf("expected project %q not found in list", name)
}
}

func TestBundleListIDs(t *testing.T) {
result := ToolBundleList()
if result.Error != "" {
t.Fatalf("ToolBundleList returned error: %s", result.Error)
}
bundles, ok := result.Data.([]Bundle)
if !ok {
t.Fatal("ToolBundleList data is not []Bundle")
}
expectedIDs := map[string]string{
"semio/js":           emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js",
"semio/engine":       emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "engine",
"semio/go":           emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "go",
"semio/rs":           emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "rs",
"semio/py":           emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "py",
"semio/net":          emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "net",
"semio/graphql":      emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleSchema) + "graphql",
"semio/jsonschema":   emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleSchema) + "jsonschema",
"semio/openapi":      emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleSchema) + "openapi",
"semio/desktop":      emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleUI) + "desktop",
"semio/docs":         emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleSite) + "docs",
"semio/play":         emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleSite) + "play",
"semio/assets":       emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleAssets) + "assets",
"semio-repo/cli":     emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleBinary) + "cli",
"semio-repo/server":  emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleBinary) + "server",
"semio-repo/go":      emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleLibrary) + "go",
"semio-repo/vscode":  emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleUI) + "vscode",
"semio-repo/graphql": emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleSchema) + "graphql",
}
for _, b := range bundles {
expected, ok := expectedIDs[b.Name]
if !ok {
continue
}
got := b.GetID()
if got != expected {
t.Errorf("bundle %q id: expected %q, got %q", b.Name, expected, got)
}
delete(expectedIDs, b.Name)
}
for name := range expectedIDs {
t.Errorf("expected bundle %q not found in list", name)
}
}

func TestSectionListIDs(t *testing.T) {
result := ToolSectionList("semio-repo/cli/main.go")
if result.Error != "" {
t.Fatalf("ToolSectionList returned error: %s", result.Error)
}
sections, ok := result.Data.([]Section)
if !ok {
t.Fatal("ToolSectionList data is not []Section")
}
if len(sections) == 0 {
t.Fatal("ToolSectionList returned no sections")
}
for _, s := range sections {
localID := s.GetID()
expectedPrefix := emojiText(EmojiSection)
if !strings.HasPrefix(localID, expectedPrefix) {
t.Errorf("section %q local id %q should start with section emoji %q", s.Name, localID, expectedPrefix)
}
flatName := Flat(s.Name)
expectedID := expectedPrefix + flatName
if localID != expectedID {
t.Errorf("section %q local id: expected %q, got %q", s.Name, expectedID, localID)
}
}
}

func TestContributorListIDs(t *testing.T) {
result := ToolContributorList()
if result.Error != "" {
t.Fatalf("ToolContributorList returned error: %s", result.Error)
}
contributors, ok := result.Data.([]Contributor)
if !ok {
t.Fatal("ToolContributorList data is not []Contributor")
}
if len(contributors) == 0 {
t.Fatal("ToolContributorList returned no contributors")
}
for _, c := range contributors {
id := c.GetID()
expectedPrefix := emojiText(EmojiContributor)
if !strings.HasPrefix(id, expectedPrefix) {
t.Errorf("contributor %q id %q should start with %q", c.Github, id, expectedPrefix)
}
expectedID := expectedPrefix + Flat(c.Github)
if id != expectedID {
t.Errorf("contributor %q id: expected %q, got %q", c.Github, expectedID, id)
}
}
foundUsalu := false
for _, c := range contributors {
if c.Github == "usalu" {
if c.GetID() != emojiText(EmojiContributor)+"usalu" {
t.Errorf("usalu contributor id: expected %q, got %q", emojiText(EmojiContributor)+"usalu", c.GetID())
}
foundUsalu = true
}
}
if !foundUsalu {
t.Error("expected to find contributor 'usalu'")
}
}

func TestGoalListIDs(t *testing.T) {
result := ToolGoalList()
if result.Error != "" {
t.Skipf("ToolGoalList returned error (may be due to existing data): %s", result.Error)
}
goals, ok := result.Data.([]Goal)
if !ok {
t.Skip("ToolGoalList data is not []Goal")
}
for _, g := range goals {
id := g.GetID()
expectedPrefix := emojiText(EmojiGoal)
if !strings.HasPrefix(id, expectedPrefix) {
t.Errorf("goal %q id %q should start with %q", g.ID, id, expectedPrefix)
}
}
}

func TestTicketListIDs(t *testing.T) {
result := ToolTicketList(nil, nil, nil)
if result.Error != "" {
t.Skipf("ToolTicketList returned error: %s", result.Error)
}
tickets, ok := result.Data.([]Ticket)
if !ok {
t.Skip("ToolTicketList data is not []Ticket")
}
for _, tk := range tickets {
id := tk.GetID()
expectedPrefix := emojiText(EmojiTicket)
if !strings.HasPrefix(id, expectedPrefix) {
t.Errorf("ticket %q id %q should start with %q", tk.Slug, id, expectedPrefix)
}
expectedID := expectedPrefix + Flat(tk.Slug)
if id != expectedID {
t.Errorf("ticket %q id: expected %q, got %q", tk.Slug, expectedID, id)
}
}
}

func TestDraftListIDs(t *testing.T) {
result := ToolDraftList()
if result.Error != "" {
t.Skipf("ToolDraftList returned error: %s", result.Error)
}
drafts, ok := result.Data.([]*Draft)
if !ok {
t.Skip("ToolDraftList data is not []*Draft")
}
for _, d := range drafts {
id := d.GetID()
expectedPrefix := emojiText(EmojiDraft)
if !strings.HasPrefix(id, expectedPrefix) {
t.Errorf("draft %q id %q should start with %q", d.ID, id, expectedPrefix)
}
}
}

func TestAllSpecIDExamples(t *testing.T) {
cases := []struct {
name     string
kind     string
data     map[string]interface{}
expected string
}{
{"root id is empty", "root", map[string]interface{}{}, ""},
{"years under root", "years", map[string]interface{}{"parentId": ""}, "\U0001F386"},
{"year 26", "year", map[string]interface{}{"parentId": "", "yy": "26"}, "\U0001F38626"},
{"months under year", "months", map[string]interface{}{"parentId": "\U0001F38626"}, "\U0001F38626\U0001F319"},
{"month 02", "month", map[string]interface{}{"parentId": "\U0001F38626", "mm": "02"}, "\U0001F38626\U0001F31902"},
{"days under month", "days", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay)},
{"day 15", "day", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902", "dd": "15"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15"},
{"hours under day", "hours", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F0"},
{"hour 14", "hour", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15", "hh": "14"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014"},
{"minutes under hour", "minutes", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014\u231B"},
{"minute 33", "minute", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014", "mm": "33"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014\u231B33"},
{"seconds under minute", "seconds", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014\u231B33"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014\u231B33" + emojiText(EmojiSecond)},
{"second 38", "second", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014\u231B33", "ss": "38"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014\u231B33" + emojiText(EmojiSecond) + "38"},
{"projects under root", "projects", map[string]interface{}{"parentId": ""}, emojiText(EmojiProjects)},
{"infra project semio-repo", "project", map[string]interface{}{"name": "semio-repo", "kind": "infrastructure"}, emojiText(EmojiProjectInfra) + "semiorepo"},
{"user project semio", "project", map[string]interface{}{"name": "semio", "kind": "user"}, emojiText(EmojiProjectUser) + "semio"},
{"research project coda", "project", map[string]interface{}{"name": "coda", "kind": "research"}, emojiText(EmojiProjectResearch) + "coda"},
{"bundles under project", "bundles", map[string]interface{}{"parentId": emojiText(EmojiProjectUser) + "semio"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundles)},
{"library bundle semio/js", "bundle", map[string]interface{}{"name": "semio/js", "kind": "library"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js"},
{"schema bundle semio/graphql", "bundle", map[string]interface{}{"name": "semio/graphql", "kind": "schema"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleSchema) + "graphql"},
{"binary bundle semio-repo/cli", "bundle", map[string]interface{}{"name": "semio-repo/cli", "kind": "binary"}, emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleBinary) + "cli"},
{"ui bundle semio-repo/vscode", "bundle", map[string]interface{}{"name": "semio-repo/vscode", "kind": "ui"}, emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleUI) + "vscode"},
{"example bundle coda/examples", "bundle", map[string]interface{}{"name": "coda/examples", "kind": "example"}, emojiText(EmojiProjectResearch) + "coda" + emojiText(EmojiBundleExample) + "examples"},
{"assets bundle semio/assets", "bundle", map[string]interface{}{"name": "semio/assets", "kind": "assets"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleAssets) + "assets"},
{"root folders", "folders", map[string]interface{}{"parentId": ""}, emojiText(EmojiFolders)},
{"org folder semio/js/sketchpad", "folder", map[string]interface{}{"path": "semio/js/sketchpad", "kind": "organization", "parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad"},
{"required folder .devcontainer", "folder", map[string]interface{}{"path": ".devcontainer", "kind": "required", "parentId": ""}, emojiText(EmojiFolderRequired) + "devcontainer"},
{"root files", "files", map[string]interface{}{"parentId": ""}, emojiText(EmojiFiles)},
{"code file Design.tsx", "file", map[string]interface{}{"path": "semio/js/sketchpad/Design.tsx", "kind": "code", "parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx"},
{"config file devcontainer.json", "file", map[string]interface{}{"path": ".devcontainer/devcontainer.json", "kind": "config", "parentId": emojiText(EmojiFolderRequired) + "devcontainer"}, emojiText(EmojiFolderRequired) + "devcontainer" + emojiText(EmojiFileConfig) + "devcontainerjson"},
{"line 3872", "line", map[string]interface{}{"parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx", "line": float64(3872)}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiLine) + "3872"},
{"range 3872-3875", "range", map[string]interface{}{"parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designstorets", "startLine": float64(3872), "endLine": float64(3875)}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designstorets" + emojiText(EmojiLine) + "3872" + emojiText(EmojiLine) + "3875"},
{"section State Managment", "section", map[string]interface{}{"name": "State Managment", "parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment"},
{"nested section Store", "section", map[string]interface{}{"name": "Store", "parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"},
{"definition impl createSketchpadStore", "definition", map[string]interface{}{"name": "createSketchpadStore", "kind": "implementation", "parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitionImpl) + "createsketchpadstore"},
{"top-level goal", "goal", map[string]interface{}{"id": "R26-02-1", "parentId": ""}, emojiText(EmojiGoal) + "r26021"},
{"nested goal Running Sketchpad", "goal", map[string]interface{}{"id": "R26-02-1/RUNNING-SKETCHPAD", "parentId": emojiText(EmojiGoal) + "r26021"}, emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad"},
{"ticket Introduce Key Guid Uri Mechanism", "ticket", map[string]interface{}{"slug": "INTRODUCE-KEY-GUID-URI-MECHANISM", "parentId": emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad"}, emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism"},
{"draft New Architecture", "draft", map[string]interface{}{"slug": "NEW-ARCHITECTURE", "parentId": emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleBinary) + "cli"}, emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleBinary) + "cli" + emojiText(EmojiDraft) + "newarchitecture"},
{"todo Introduce Proper Sync Mechanism", "todo", map[string]interface{}{"id": "INTRODUCE-PROPER-SYNC-MECHANISM", "parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitionImpl) + "createsketchpadstore"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitionImpl) + "createsketchpadstore" + emojiText(EmojiTodo) + "introducepropersyncmechanism"},
{"general policy godfiles", "policy", map[string]interface{}{"id": "godfiles", "parentId": emojiText(EmojiFileCode)}, emojiText(EmojiFileCode) + emojiText(EmojiPolicy) + "godfiles"},
{"contributor usalu", "contributor", map[string]interface{}{"github": "usalu"}, emojiText(EmojiContributor) + "usalu"},
{"commit", "commit", map[string]interface{}{"sha": "cfb3b6084ff3fe883d5f39b08810a0b90997907a", "contributorId": emojiText(EmojiContributor) + "usalu"}, emojiText(EmojiContributor) + "usalu" + emojiText(EmojiCommit) + "cfb3b6084ff3fe883d5f39b08810a0b90997907a"},
{"interaction started", "interaction", map[string]interface{}{
"secondId":      emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12",
"contributorId": emojiText(EmojiContributor) + "usalu",
"entityId":      emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism",
"kind":          "started",
}, emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12" + emojiText(EmojiContributor) + "usalu" + emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism" + emojiText(EmojiInteractionStarted)},
{"site bundle semio/docs", "bundle", map[string]interface{}{"name": "semio/docs", "kind": "site"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleSite) + "docs"},
}
for _, tc := range cases {
t.Run(tc.name, func(t *testing.T) {
id := GetArtifactID(tc.kind, tc.data)
if id != tc.expected {
t.Errorf("%s: expected %q, got %q", tc.name, tc.expected, id)
}
})
}
}
