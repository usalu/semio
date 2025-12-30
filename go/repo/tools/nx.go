// repo/tools/nx.go

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

package tools

import (
	"encoding/json"
	"strings"
	"sync"
)

var (
	cachedProjectNames   []string
	cachedProjectDetails = make(map[string]NxProject)
	nxMutex              sync.Mutex
)

func GetNxProjectNames() []string {
	nxMutex.Lock()
	defer nxMutex.Unlock()
	if cachedProjectNames != nil {
		return cachedProjectNames
	}
	stdout, _, exitCode := ExecCommand("npx", []string{"nx", "show", "projects", "--json"}, "")
	if exitCode != 0 {
		cachedProjectNames = []string{}
		return cachedProjectNames
	}
	var names []string
	if err := json.Unmarshal([]byte(stdout), &names); err != nil {
		cachedProjectNames = []string{}
		return cachedProjectNames
	}
	cachedProjectNames = names
	return cachedProjectNames
}

func GetNxProjectDetails(name string) NxProject {
	nxMutex.Lock()
	defer nxMutex.Unlock()
	if proj, ok := cachedProjectDetails[name]; ok {
		return proj
	}
	stdout, _, exitCode := ExecCommand("npx", []string{"nx", "show", "project", name, "--json"}, "")
	if exitCode != 0 {
		proj := NxProject{Name: name}
		cachedProjectDetails[name] = proj
		return proj
	}
	var config map[string]interface{}
	if err := json.Unmarshal([]byte(stdout), &config); err != nil {
		proj := NxProject{Name: name}
		cachedProjectDetails[name] = proj
		return proj
	}
	proj := NxProject{Name: name}
	if root, ok := config["root"].(string); ok {
		proj.Root = root
	}
	if sourceRoot, ok := config["sourceRoot"].(string); ok {
		proj.SourceRoot = sourceRoot
	}
	if projectType, ok := config["projectType"].(string); ok {
		proj.ProjectType = projectType
	}
	if tags, ok := config["tags"].([]interface{}); ok {
		for _, t := range tags {
			if tag, ok := t.(string); ok {
				proj.Tags = append(proj.Tags, tag)
			}
		}
	}
	cachedProjectDetails[name] = proj
	return proj
}

func GetNxProjects() []NxProject {
	names := GetNxProjectNames()
	projects := make([]NxProject, len(names))
	for i, name := range names {
		projects[i] = GetNxProjectDetails(name)
	}
	return projects
}

func RunNxTarget(target string, projects []string, extraArgs []string) (success bool, output string) {
	args := []string{"nx"}
	if len(projects) == 1 {
		args = append(args, "run", projects[0]+":"+target)
	} else if len(projects) > 1 {
		args = append(args, "run-many", "-t", target, "-p", strings.Join(projects, ","))
	} else {
		args = append(args, "run-many", "-t", target)
	}
	args = append(args, extraArgs...)
	stdout, stderr, exitCode := ExecCommand("npx", args, "")
	return exitCode == 0, stdout + stderr
}

func ScopeToFiles(scope Scope, projects []NxProject) ([]string, error) {
	ignorePatterns := []string{"**/node_modules/**", "**/.venv/**"}
	switch scope.Kind {
	case ScopeRepo:
		return SimpleGlob("**/*.{ts,tsx,py,cs}", rootDir, ignorePatterns, true)
	case ScopeProject:
		for _, proj := range projects {
			if proj.Name == scope.ProjectName {
				return SimpleGlob(proj.Root+"/**/*.{ts,tsx,py,cs}", rootDir, ignorePatterns, true)
			}
		}
		return nil, nil
	case ScopeFolder:
		if scope.FilePath != "" {
			return SimpleGlob(scope.FilePath+"**/*.{ts,tsx,py,cs}", rootDir, ignorePatterns, true)
		}
		return nil, nil
	case ScopeFile, ScopeSection, ScopeDefinition:
		if scope.FilePath != "" {
			return []string{scope.FilePath}, nil
		}
		return nil, nil
	default:
		return nil, nil
	}
}

