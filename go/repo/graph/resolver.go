// #region Header

// go/repo/graph/resolver.go

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion Header

package graph

// #region Context Interface

type RepoContext interface {
	GetRootDir() string
	GetBundles() []*Bundle
	GetContributors() ([]*Contributor, error)
	GetTickets(year, month, day *int, status *TicketStatus) ([]*Ticket, error)
	GetPolicies() []*Policy
	GetViolationKinds() []*ViolationKind
	Analyze(scope *string) (*AnalyzeResult, error)
	Fix(scope *string) (*FixResult, error)
	TicketCreate(input TicketCreateInput) (*Ticket, error)
	TicketProgress(input TicketProgressInput) (*Ticket, error)
	TicketFinish(input TicketFinishInput) (*Ticket, error)
	TicketReopen(input TicketReopenInput) (*Ticket, error)
	FolderCreate(path string) (*Folder, error)
	FolderMove(src, dst string) (*Folder, error)
	FolderDelete(path string) error
	FileCreate(path string) (*File, error)
	FileMove(src, dst string) (*File, error)
	FileDelete(path string) error
	SectionCreate(file, name string, parent *string) (*Section, error)
	SectionMove(file, oldName, newName string) (*Section, error)
	SectionDelete(file, name string) error
	ContributorAdd(input ContributorAddInput) (*Contributor, error)
	ContributorRemove(github string) error
}

// #endregion Context Interface

// #region Resolver

type Resolver struct {
	RootDir string
	Ctx     RepoContext
}

func NewResolver(rootDir string) *Resolver {
	return &Resolver{RootDir: rootDir}
}

func NewResolverWithContext(rootDir string, ctx RepoContext) *Resolver {
	return &Resolver{RootDir: rootDir, Ctx: ctx}
}

func (r *Resolver) context() RepoContext {
	return r.Ctx
}

// #endregion Resolver

// #region Default Context

type defaultContext struct {
	rootDir string
}

func NewDefaultContext(rootDir string) RepoContext {
	return &defaultContext{rootDir: rootDir}
}

func (c *defaultContext) GetRootDir() string { return c.rootDir }

func (c *defaultContext) GetBundles() []*Bundle { return []*Bundle{} }

func (c *defaultContext) GetContributors() ([]*Contributor, error) { return []*Contributor{}, nil }

func (c *defaultContext) GetTickets(year, month, day *int, status *TicketStatus) ([]*Ticket, error) {
	return []*Ticket{}, nil
}

func (c *defaultContext) GetPolicies() []*Policy { return []*Policy{} }

func (c *defaultContext) GetViolationKinds() []*ViolationKind { return []*ViolationKind{} }

func (c *defaultContext) Analyze(scope *string) (*AnalyzeResult, error) {
	return &AnalyzeResult{Violations: []*Violation{}, Metrics: &AnalyzeMetrics{}}, nil
}

func (c *defaultContext) Fix(scope *string) (*FixResult, error) {
	return &FixResult{Violations: []*Violation{}}, nil
}

func (c *defaultContext) TicketCreate(input TicketCreateInput) (*Ticket, error) {
	return nil, nil
}

func (c *defaultContext) TicketProgress(input TicketProgressInput) (*Ticket, error) {
	return nil, nil
}

func (c *defaultContext) TicketFinish(input TicketFinishInput) (*Ticket, error) {
	return nil, nil
}

func (c *defaultContext) TicketReopen(input TicketReopenInput) (*Ticket, error) {
	return nil, nil
}

func (c *defaultContext) FolderCreate(path string) (*Folder, error) { return nil, nil }

func (c *defaultContext) FolderMove(src, dst string) (*Folder, error) { return nil, nil }

func (c *defaultContext) FolderDelete(path string) error { return nil }

func (c *defaultContext) FileCreate(path string) (*File, error) { return nil, nil }

func (c *defaultContext) FileMove(src, dst string) (*File, error) { return nil, nil }

func (c *defaultContext) FileDelete(path string) error { return nil }

func (c *defaultContext) SectionCreate(file, name string, parent *string) (*Section, error) {
	return nil, nil
}

func (c *defaultContext) SectionMove(file, oldName, newName string) (*Section, error) {
	return nil, nil
}

func (c *defaultContext) SectionDelete(file, name string) error { return nil }

func (c *defaultContext) ContributorAdd(input ContributorAddInput) (*Contributor, error) {
	return nil, nil
}

func (c *defaultContext) ContributorRemove(github string) error { return nil }

var _ RepoContext = (*defaultContext)(nil)

// #endregion Default Context
