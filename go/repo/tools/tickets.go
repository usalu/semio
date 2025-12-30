// repo/tools/tickets.go

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
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strconv"
	"strings"
	"time"

	"gopkg.in/yaml.v3"
)

func GetTicketsDir() string {
	return filepath.Join(rootDir, "tickets")
}

func GetTicketPath(year, month, day int, slug string) string {
	return filepath.Join(GetTicketsDir(), strconv.Itoa(year), PadNumber(month, 2), PadNumber(day, 2), slug+".md")
}

func CreateTicket(slug, prompt, model string) (*Ticket, error) {
	now := time.Now()
	year, month, day := FormatDate(now)
	normalizedSlug := Slugify(slug)
	filePath := GetTicketPath(year, month, day, normalizedSlug)
	gitAuthor := GetGitAuthor()
	gitCommit := GetGitCommit()
	frontmatter := TicketFrontmatter{
		Slug:   normalizedSlug,
		Prompt: prompt,
		Status: TicketOpen,
		Author: gitAuthor,
		Date:   TicketDateCreated{Created: ISOTimestamp()},
		Commit: gitCommit,
		Model:  model,
	}
	content := `# Previously

# Plan

# Changes`
	ticket := &Ticket{
		Year:        year,
		Month:       month,
		Day:         day,
		Slug:        normalizedSlug,
		Frontmatter: frontmatter,
		Content:     content,
		FilePath:    filePath,
	}
	if err := SaveTicket(ticket); err != nil {
		return nil, err
	}
	return ticket, nil
}

func ReadTicket(year, month, day int, slug string) (*Ticket, error) {
	filePath := GetTicketPath(year, month, day, slug)
	if !FileExists(filePath) {
		return nil, fmt.Errorf("ticket not found: %s", filePath)
	}
	raw, err := ReadTextFile(filePath)
	if err != nil {
		return nil, err
	}
	frontmatter, content, err := parseFrontmatter(raw)
	if err != nil {
		return nil, err
	}
	return &Ticket{
		Year:        year,
		Month:       month,
		Day:         day,
		Slug:        slug,
		Frontmatter: frontmatter,
		Content:     content,
		FilePath:    filePath,
	}, nil
}

func parseFrontmatter(raw string) (TicketFrontmatter, string, error) {
	var frontmatter TicketFrontmatter
	if !strings.HasPrefix(raw, "---") {
		return frontmatter, raw, nil
	}
	endIndex := strings.Index(raw[3:], "---")
	if endIndex == -1 {
		return frontmatter, raw, nil
	}
	yamlContent := raw[3 : endIndex+3]
	content := strings.TrimPrefix(raw[endIndex+6:], "\n")
	if err := yaml.Unmarshal([]byte(yamlContent), &frontmatter); err != nil {
		return frontmatter, content, err
	}
	return frontmatter, content, nil
}

func SaveTicket(ticket *Ticket) error {
	yamlBytes, err := yaml.Marshal(ticket.Frontmatter)
	if err != nil {
		return err
	}
	content := fmt.Sprintf("---\n%s---\n%s", string(yamlBytes), ticket.Content)
	return WriteTextFile(ticket.FilePath, content)
}

func ListTickets(year, month, day *int) ([]Ticket, error) {
	ticketsDir := GetTicketsDir()
	if !FileExists(ticketsDir) {
		return nil, nil
	}
	var tickets []Ticket
	var years []string
	if year != nil {
		years = []string{strconv.Itoa(*year)}
	} else {
		entries, err := os.ReadDir(ticketsDir)
		if err != nil {
			return nil, err
		}
		for _, e := range entries {
			if e.IsDir() {
				years = append(years, e.Name())
			}
		}
	}
	for _, y := range years {
		yearPath := filepath.Join(ticketsDir, y)
		if !FileExists(yearPath) {
			continue
		}
		var months []string
		if month != nil {
			months = []string{PadNumber(*month, 2)}
		} else {
			entries, err := os.ReadDir(yearPath)
			if err != nil {
				continue
			}
			for _, e := range entries {
				if e.IsDir() {
					months = append(months, e.Name())
				}
			}
		}
		for _, m := range months {
			monthPath := filepath.Join(yearPath, m)
			if !FileExists(monthPath) {
				continue
			}
			var days []string
			if day != nil {
				days = []string{PadNumber(*day, 2)}
			} else {
				entries, err := os.ReadDir(monthPath)
				if err != nil {
					continue
				}
				for _, e := range entries {
					if e.IsDir() {
						days = append(days, e.Name())
					}
				}
			}
			for _, d := range days {
				dayPath := filepath.Join(monthPath, d)
				if !FileExists(dayPath) {
					continue
				}
				entries, err := os.ReadDir(dayPath)
				if err != nil {
					continue
				}
				for _, e := range entries {
					if !e.IsDir() && strings.HasSuffix(e.Name(), ".md") {
						slug := strings.TrimSuffix(e.Name(), ".md")
						yearInt, _ := strconv.Atoi(y)
						monthInt, _ := strconv.Atoi(m)
						dayInt, _ := strconv.Atoi(d)
						ticket, err := ReadTicket(yearInt, monthInt, dayInt, slug)
						if err == nil {
							tickets = append(tickets, *ticket)
						}
					}
				}
			}
		}
	}
	return tickets, nil
}

func StartIteration(ticket *Ticket, prompt, model string) error {
	gitAuthor := GetGitAuthor()
	iteration := TicketIteration{
		Prompt: prompt,
		Model:  model,
		Date:   TicketDate{Started: ISOTimestamp()},
		Author: gitAuthor,
	}
	ticket.Frontmatter.Iterations = append(ticket.Frontmatter.Iterations, iteration)
	return SaveTicket(ticket)
}

func EndIteration(ticket *Ticket) error {
	if len(ticket.Frontmatter.Iterations) == 0 {
		return fmt.Errorf("no active iteration to end")
	}
	lastIdx := len(ticket.Frontmatter.Iterations) - 1
	last := &ticket.Frontmatter.Iterations[lastIdx]
	if last.Date.Ended != "" {
		return fmt.Errorf("last iteration already ended")
	}
	last.Date.Ended = ISOTimestamp()
	last.Commit = GetGitCommit()
	return SaveTicket(ticket)
}

func FinishTicket(ticket *Ticket) error {
	if len(ticket.Frontmatter.Iterations) > 0 {
		last := ticket.Frontmatter.Iterations[len(ticket.Frontmatter.Iterations)-1]
		if last.Date.Ended == "" {
			return fmt.Errorf("cannot finish ticket with unfinished iteration")
		}
	}
	ticket.Frontmatter.Status = TicketClosed
	ticket.Frontmatter.Date.Finished = ISOTimestamp()
	return SaveTicket(ticket)
}

func CanCloseTicket(ticket *Ticket) (bool, []string) {
	var reasons []string
	violationsRe := regexp.MustCompile(`(?s)## Violations.*?(?:\n## |$)`)
	if match := violationsRe.FindString(ticket.Content); match != "" {
		if !strings.Contains(match, "(No violations)") && strings.Contains(match, "- [") {
			reasons = append(reasons, "Violations section is not empty")
		}
	}
	planRe := regexp.MustCompile(`(?s)# Plan.*?(?:\n# |$)`)
	if match := planRe.FindString(ticket.Content); match == "" || strings.TrimSpace(match) == "# Plan" {
		reasons = append(reasons, "Plan section is empty")
	}
	changesRe := regexp.MustCompile(`(?s)# Changes.*?(?:\n# |$)`)
	if match := changesRe.FindString(ticket.Content); match == "" || strings.TrimSpace(match) == "# Changes" {
		reasons = append(reasons, "Changes section is empty")
	}
	return len(reasons) == 0, reasons
}

