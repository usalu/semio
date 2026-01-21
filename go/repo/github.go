package repo

import (
	"fmt"
	"os"
	"sort"
	"strings"
)

func GitHubCreateIssue(title, body string, labels []string) (string, error) {
	args := []string{"issue", "create", "--title", title, "--body", body}
	for _, l := range labels {
		args = append(args, "--label", l)
	}
	stdout, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return "", fmt.Errorf("gh issue create failed: %s", stderr)
	}
	return strings.TrimSpace(stdout), nil
}

func GitHubAddToProject(issueUrl string, projectNumber int, owner string) error {
	args := []string{"project", "item-add", fmt.Sprintf("%d", projectNumber), "--owner", owner, "--url", issueUrl}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh project item-add failed: %s", stderr)
	}
	return nil
}

func GitHubCloseIssue(issueUrl string) error {
	_, stderr, exitCode := ExecCommand("gh", []string{"issue", "close", issueUrl}, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue close failed: %s", stderr)
	}
	return nil
}

func GitHubReopenIssue(issueUrl string) error {
	_, stderr, exitCode := ExecCommand("gh", []string{"issue", "reopen", issueUrl}, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue reopen failed: %s", stderr)
	}
	return nil
}

func GitHubCommentIssue(issueUrl, body string) error {
	tmpFile, err := os.CreateTemp("", "gh-comment-*.md")
	if err != nil {
		return err
	}
	defer os.Remove(tmpFile.Name())
	if _, err := tmpFile.WriteString(body); err != nil {
		return err
	}
	tmpFile.Close()

	_, stderr, exitCode := ExecCommand("gh", []string{"issue", "comment", issueUrl, "--body-file", tmpFile.Name()}, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue comment failed: %s", stderr)
	}
	return nil
}

func GitHubAddLabels(issueUrl string, labels []string) error {
	if len(labels) == 0 {
		return nil
	}
	args := []string{"issue", "edit", issueUrl}
	for _, l := range labels {
		args = append(args, "--add-label", l)
	}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue edit failed: %s", stderr)
	}
	return nil
}

func GenerateMetricsComment(files []TicketFile) string {
	var lines []string
	lines = append(lines, "```md")

	// Sort files by path
	sortedFiles := make([]TicketFile, len(files))
	copy(sortedFiles, files)
	sort.Slice(sortedFiles, func(i, j int) bool {
		return sortedFiles[i].Path < sortedFiles[j].Path
	})

	for _, f := range sortedFiles {
		added := 0
		removed := 0
		for _, s := range f.Sections {
			if s.Lines != nil {
				added += s.Lines.Added
				removed += s.Lines.Removed
			}
		}

		icon := "✏️"
		if f.Status == "created" {
			icon = "➕"
		} else if f.Status == "removed" {
			icon = "➖"
		}

		lineStr := ""
		if added > 0 {
			lineStr += fmt.Sprintf(" +%d", added)
		}
		if removed > 0 {
			lineStr += fmt.Sprintf(" -%d", removed)
		}

		// For deleted files, we might show total lines as removed if we knew them,
		// but here we rely on what's in Sections.
		// If sections are empty for removed file, lineStr might be empty.

		lines = append(lines, fmt.Sprintf("%s%s%s", icon, f.Path, lineStr))
	}
	lines = append(lines, "```")
	return strings.Join(lines, "\n")
}
