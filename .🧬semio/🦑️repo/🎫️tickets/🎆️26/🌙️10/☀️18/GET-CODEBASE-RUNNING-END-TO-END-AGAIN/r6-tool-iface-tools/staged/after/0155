// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🔒️git carries the git version control provider and the git diff queries.

// #endregion 🧲️Header

package providers

import (
	fmt "fmt"
	regexp "regexp"
	strconv "strconv"
	strings "strings"
	time "time"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🔒️Git

// 🔌️GitVersionControlProvider holds the data fields for a git version control provider record.
type GitVersionControlProvider struct{}

// 💿️Kind holds the data fields for a Kind record.
func (p *GitVersionControlProvider) Kind() string { return "git" }

// 🌐️RepoURL holds the data fields for a RepoURL record.
func (p *GitVersionControlProvider) RepoURL() (string, error) {
	out, err := workspace.ToolCommand("gh", "repo", "view", "--json", "url", "--jq", ".url").Output()
	if err != nil {
		return "", err
	}
	return strings.TrimSpace(string(out)), nil
}

// ⚙️Configure holds the data fields for a Configure record.
func (p *GitVersionControlProvider) Configure(repoRoot string) error { return nil }

// 💾️Checkpoint holds the data fields for a Checkpoint record.
func (p *GitVersionControlProvider) Checkpoint(repoRoot string, description string) (string, error) {
	if _, _, exitCode := workspace.ExecCommand("git", []string{"diff", "--cached", "--quiet"}, repoRoot); exitCode == 0 {
		if err := p.StageAll(repoRoot); err != nil {
			return "", fmt.Errorf("stage all failed: %w", err)
		}
	}
	args := []string{"commit", "-m", description}
	stdout, stderr, exitCode := workspace.ExecCommand("git", args, repoRoot)
	if exitCode != 0 {
		return "", fmt.Errorf("git commit failed (exit %d): %s", exitCode, strings.TrimSpace(stderr))
	}
	_ = stdout
	sha, err := p.CurrentCheckpoint(repoRoot)
	if err != nil {
		return "", fmt.Errorf("failed to get checkpoint sha after checkpoint: %w", err)
	}
	return sha, nil
}

func (p *GitVersionControlProvider) CurrentCheckpoint(repoRoot string) (string, error) {
	stdout, stderr, exitCode := workspace.ExecCommand("git", []string{"rev-parse", "HEAD"}, repoRoot)
	if exitCode != 0 {
		return "", fmt.Errorf("git rev-parse HEAD failed: %s", strings.TrimSpace(stderr))
	}
	return strings.TrimSpace(stdout), nil
}

// ✔️Checkin holds the data fields for a Checkin record.
func (p *GitVersionControlProvider) Checkin(repoRoot string, contributor string) error {
	contributorBranch := contributor + "/latest"

	_, stderr, exitCode := workspace.ExecCommand("git", []string{"fetch", "origin", "main"}, repoRoot)
	if exitCode != 0 {
		return fmt.Errorf("git fetch origin main failed: %s", strings.TrimSpace(stderr))
	}

	currentBranch, err := p.CurrentBranch(repoRoot)
	if err != nil {
		return err
	}

	if currentBranch != contributorBranch {
		_, stderr, exitCode = workspace.ExecCommand("git", []string{"switch", contributorBranch}, repoRoot)
		if exitCode != 0 {

			_, stderr, exitCode = workspace.ExecCommand("git", []string{"switch", "-c", contributorBranch}, repoRoot)
			if exitCode != 0 {
				return fmt.Errorf("git switch to %s failed: %s", contributorBranch, strings.TrimSpace(stderr))
			}
		}
	}

	_, stderr, exitCode = workspace.ExecCommand("git", []string{"merge", "--ff-only", "origin/main"}, repoRoot)
	if exitCode != 0 {
		return fmt.Errorf("git fast-forward merge to main failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// 🔷️Checkout holds the data fields for a Checkout record.
func (p *GitVersionControlProvider) Checkout(repoRoot string, contributor string, description string) (string, error) {
	contributorBranch := contributor + "/latest"
	now := time.Now().UTC()
	archiveBranch := fmt.Sprintf("%s/%04d/%02d/%02d", contributor, now.Year(), now.Month(), now.Day())

	_, stderr, exitCode := workspace.ExecCommand("git", []string{"branch", archiveBranch, contributorBranch}, repoRoot)
	if exitCode != 0 {

		for i := 2; i <= 99; i++ {
			candidate := fmt.Sprintf("%s-%d", archiveBranch, i)
			_, _, ec := workspace.ExecCommand("git", []string{"branch", candidate, contributorBranch}, repoRoot)
			if ec == 0 {
				archiveBranch = candidate
				break
			}
		}
	}

	_, stderr, exitCode = workspace.ExecCommand("git", []string{"switch", "main"}, repoRoot)
	if exitCode != 0 {
		return "", fmt.Errorf("git switch to main failed: %s", strings.TrimSpace(stderr))
	}

	_, stderr, exitCode = workspace.ExecCommand("git", []string{"merge", "--squash", contributorBranch}, repoRoot)
	if exitCode != 0 {
		return "", fmt.Errorf("git squash merge failed: %s", strings.TrimSpace(stderr))
	}

	_, stderr, exitCode = workspace.ExecCommand("git", []string{"commit", "-m", description}, repoRoot)
	if exitCode != 0 {
		return "", fmt.Errorf("git commit squash merge failed: %s", strings.TrimSpace(stderr))
	}

	sha, err := p.CurrentCheckpoint(repoRoot)
	if err != nil {
		return "", err
	}
	return sha, nil
}

// 🌿️CurrentBranch holds the data fields for a CurrentBranch record.
func (p *GitVersionControlProvider) CurrentBranch(repoRoot string) (string, error) {
	stdout, stderr, exitCode := workspace.ExecCommand("git", []string{"rev-parse", "--abbrev-ref", "HEAD"}, repoRoot)
	if exitCode != 0 {
		return "", fmt.Errorf("git rev-parse --abbrev-ref HEAD failed: %s", strings.TrimSpace(stderr))
	}
	return strings.TrimSpace(stdout), nil
}

// 📄️StagedFiles holds the data fields for a StagedFiles record.
func (p *GitVersionControlProvider) StagedFiles(repoRoot string) ([]string, error) {
	stdout, stderr, exitCode := workspace.ExecCommand("git", []string{"diff", "--cached", "--name-only"}, repoRoot)
	if exitCode != 0 {
		return nil, fmt.Errorf("git diff --cached --name-only failed: %s", strings.TrimSpace(stderr))
	}
	var files []string
	for _, line := range strings.Split(strings.ReplaceAll(stdout, "\r\n", "\n"), "\n") {
		if line != "" {
			files = append(files, line)
		}
	}
	return files, nil
}

// 🏷️StageAll holds the data fields for a StageAll record.
func (p *GitVersionControlProvider) StageAll(repoRoot string) error {
	_, stderr, exitCode := workspace.ExecCommand("git", []string{"add", "-A"}, repoRoot)
	if exitCode != 0 {
		return fmt.Errorf("git add -A failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// 🌳️GitIndexRef is the git ref for the staging index. Used for unstaged-only diffs (index vs working tree).
// ⏹️Requirements: ticket close and interaction finish use only unstaged diffs; git diff runs without tree-ish for index vs working tree.
const GitIndexRef = ":0"

// 📨️GetGitDiffLines MUST retrieve the requested value or return an error.
// GetGitDiffLines retrieves and returns the git diff lines.
// ✔️For unstaged-only diffs use baseCheckpoint GitIndexRef (index vs working tree).
func GetGitDiffLines(baseCheckpoint, headCheckpoint string, paths []string) (map[string]*model.DiffLines, error) {
	if baseCheckpoint == "" {
		return nil, fmt.Errorf("base checkpoint or GitIndexRef is required")
	}
	args := BuildGitDiffArgs("-U0", baseCheckpoint, headCheckpoint, paths)
	stdout, stderr, exitCode := workspace.ExecCommand("git", args, "")
	if exitCode != 0 {
		return nil, fmt.Errorf("git diff failed: %s", strings.TrimSpace(stderr))
	}
	result := make(map[string]*model.DiffLines)
	var currentFile string
	lineRegex := regexp.MustCompile(`^@@\s+-(\d+)(?:,(\d+))?\s+\+(\d+)(?:,(\d+))?\s+@@`)
	for _, line := range strings.Split(stdout, "\n") {
		if strings.HasPrefix(line, "diff --git ") {
			parts := strings.Fields(line)
			if len(parts) >= 4 {
				newPath := strings.TrimPrefix(parts[3], "b/")
				currentFile = newPath
				if result[currentFile] == nil {
					result[currentFile] = &model.DiffLines{Added: []int{}, Removed: []int{}}
				}
			}
		} else if strings.HasPrefix(line, "+++ b/") {
			currentFile = strings.TrimPrefix(line, "+++ b/")
			if result[currentFile] == nil {
				result[currentFile] = &model.DiffLines{Added: []int{}, Removed: []int{}}
			}
		} else if strings.HasPrefix(line, "@@") && currentFile != "" {
			match := lineRegex.FindStringSubmatch(line)
			if match != nil {
				oldStart, _ := strconv.Atoi(match[1])
				oldCount := 1
				if match[2] != "" {
					oldCount, _ = strconv.Atoi(match[2])
				}
				for i := 0; i < oldCount; i++ {
					result[currentFile].Removed = append(result[currentFile].Removed, oldStart+i)
				}

				newStart, _ := strconv.Atoi(match[3])
				newCount := 1
				if match[4] != "" {
					newCount, _ = strconv.Atoi(match[4])
				}
				for i := 0; i < newCount; i++ {
					result[currentFile].Added = append(result[currentFile].Added, newStart+i)
				}
			}
		}
	}
	return result, nil
}

// 🧱️BuildGitDiffArgs MUST construct and return the fully initialized result.
// BuildGitDiffArgs constructs and returns the git diff args.
// 🌳️GitIndexRef as baseCheckpoint yields unstaged-only diff (index vs working tree) with no tree-ish.
func BuildGitDiffArgs(flag, baseCheckpoint, headCheckpoint string, paths []string) []string {
	if baseCheckpoint == GitIndexRef {
		if len(paths) == 0 {
			return []string{"diff", flag, "-M"}
		}
		return append([]string{"diff", flag, "-M", "--"}, paths...)
	}
	if headCheckpoint == "" {
		if len(paths) == 0 {
			return []string{"diff", flag, "-M", baseCheckpoint}
		}
		return append([]string{"diff", flag, "-M", baseCheckpoint, "--"}, paths...)
	}
	if len(paths) == 0 {
		return []string{"diff", flag, "-M", baseCheckpoint, headCheckpoint}
	}
	return append([]string{"diff", flag, "-M", baseCheckpoint, headCheckpoint, "--"}, paths...)
}

// 🔺️GitDiffStatus holds the data fields for a git diff status record.
type GitDiffStatus struct {
	Status string
	From   string
	To     string
}

// 🔻️GetGitDiffStatus MUST retrieve the requested value or return an error.
// 🐙️GetGitDiffStatus retrieves and returns the git diff status.
func GetGitDiffStatus(baseCheckpoint, headCheckpoint string, paths []string) ([]GitDiffStatus, error) {
	if baseCheckpoint == "" {
		return nil, fmt.Errorf("base checkpoint or GitIndexRef is required")
	}
	args := []string{"diff", "--name-status", "-M"}
	if baseCheckpoint != GitIndexRef {
		args = append(args, baseCheckpoint)
		if headCheckpoint != "" {
			args = append(args, headCheckpoint)
		}
	}
	if len(paths) > 0 {
		args = append(args, "--")
		args = append(args, paths...)
	}
	stdout, stderr, exitCode := workspace.ExecCommand("git", args, "")
	if exitCode != 0 {
		return nil, fmt.Errorf("git diff status failed: %s", strings.TrimSpace(stderr))
	}
	var results []GitDiffStatus
	for _, line := range strings.Split(strings.TrimSpace(stdout), "\n") {
		if line == "" {
			continue
		}
		parts := strings.Split(line, "\t")
		if len(parts) < 2 {
			continue
		}
		status := strings.TrimSpace(parts[0])
		if strings.HasPrefix(status, "R") && len(parts) >= 3 {
			results = append(results, GitDiffStatus{Status: "renamed", From: parts[1], To: parts[2]})
			continue
		}
		file := parts[1]
		switch status {
		case "A":
			results = append(results, GitDiffStatus{Status: "added", To: file})
		case "D":
			results = append(results, GitDiffStatus{Status: "deleted", From: file})
		case "M":
			results = append(results, GitDiffStatus{Status: "modified", To: file})
		default:
			results = append(results, GitDiffStatus{Status: "modified", To: file})
		}
	}
	return results, nil
}

// #endregion 🔒️Git
