package main

import (
	"encoding/csv"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"sort"
	"strings"
	"sync"

	"github.com/spf13/cobra"
)

var benchmarkCmd = &cobra.Command{
	Use:   "benchmark",
	Short: "Run benchmarks for all ecosystems",
	RunE:  runBenchmark,
}

func init() {
	rootCmd.AddCommand(benchmarkCmd)
}

type BenchmarkResult struct {
	Test string
	Lang string
	Time string
}

func runBenchmark(cmd *cobra.Command, args []string) error {
	rootDir := findRepoRoot(".")
	results := make([]BenchmarkResult, 0)
	var mu sync.Mutex
	var wg sync.WaitGroup

	tasks := []struct {
		Name    string
		Cmd     string
		Args    []string
		Dir     string
		Enabled bool
	}{
		{
			Name:    "Typescript",
			Cmd:     "npx",
			Args:    []string{"tsx", "benchmark.ts"},
			Dir:     filepath.Join(rootDir, "js", "semio"),
			Enabled: true,
		},
		{
			Name:    "Python",
			Cmd:     "uv",
			Args:    []string{"run", "benchmark.py"},
			Dir:     filepath.Join(rootDir, "py", "semio"),
			Enabled: true,
		},
		{
			Name:    "Go",
			Cmd:     "go",
			Args:    []string{"run", "benchmark/main.go"},
			Dir:     filepath.Join(rootDir, "go", "semio"),
			Enabled: true,
		},
		{
			Name:    "C#",
			Cmd:     "dotnet",
			Args:    []string{"run", "--project", "Semio.Benchmark/Semio.Benchmark.csproj", "--configuration", "Release"},
			Dir:     filepath.Join(rootDir, "net"),
			Enabled: true,
		},
		{
			Name:    "Rust",
			Cmd:     "cargo",
			Args:    []string{"run", "--release", "--example", "benchmark"},
			Dir:     filepath.Join(rootDir, "rs", "semio"),
			Enabled: true,
		},
	}

	fmt.Println("Running benchmarks...")

	for _, task := range tasks {
		if !task.Enabled {
			continue
		}
		wg.Add(1)
		go func(t struct {
			Name    string
			Cmd     string
			Args    []string
			Dir     string
			Enabled bool
		}) {
			defer wg.Done()
			fmt.Printf("Running %s...\n", t.Name)
			
			// Check if directory exists
			if _, err := os.Stat(t.Dir); os.IsNotExist(err) {
				fmt.Printf("Skipping %s: directory %s not found\n", t.Name, t.Dir)
				return
			}

			c := exec.Command(t.Cmd, t.Args...)
			c.Dir = t.Dir
			output, err := c.Output()
			if err != nil {
				if exitErr, ok := err.(*exec.ExitError); ok {
					fmt.Printf("%s failed: %s\n%s\n", t.Name, err, string(exitErr.Stderr))
				} else {
					fmt.Printf("%s failed: %s\n", t.Name, err)
				}
				return
			}

			mu.Lock()
			parseBenchmarkOutput(&results, t.Name, string(output))
			mu.Unlock()
		}(task)
	}

	wg.Wait()

	// Write report
	if len(results) > 0 {
		return writeBenchmarkReport(rootDir, results)
	}
	
	return nil
}

func parseBenchmarkOutput(results *[]BenchmarkResult, lang string, output string) {
	lines := strings.Split(output, "\n")
	for _, line := range lines {
		trimmed := strings.TrimSpace(line)
		if trimmed == "" {
			continue
		}
		parts := strings.Split(trimmed, ",")
		// Expected format: "TestName,TimeInSeconds"
		// Simple validation to avoid capturing logging output
		if len(parts) == 2 && 
			!strings.Contains(parts[0], "warning") && 
			!strings.Contains(parts[0], ":") && 
			!strings.Contains(parts[0], string(os.PathSeparator)) {
			*results = append(*results, BenchmarkResult{
				Test: parts[0],
				Lang: lang,
				Time: parts[1],
			})
		}
	}
}

func writeBenchmarkReport(rootDir string, results []BenchmarkResult) error {
	reportFile := filepath.Join(rootDir, "temp", "benchmark.csv")
	if err := os.MkdirAll(filepath.Dir(reportFile), 0755); err != nil {
		return err
	}

	// Collate results
	testsMap := make(map[string]bool)
	for _, r := range results {
		testsMap[r.Test] = true
	}
	var tests []string
	for t := range testsMap {
		tests = append(tests, t)
	}
	sort.Strings(tests)

	langs := []string{"Typescript", "Python", "Go", "C#", "Rust"}

	file, err := os.Create(reportFile)
	if err != nil {
		return err
	}
	defer file.Close()

	writer := csv.NewWriter(file)
	defer writer.Flush()

	// Header
	header := []string{"Test"}
	header = append(header, langs...)
	if err := writer.Write(header); err != nil {
		return err
	}

	// Rows
	for _, test := range tests {
		row := []string{test}
		for _, lang := range langs {
			timeVal := ""
			for _, r := range results {
				if r.Test == test && r.Lang == lang {
					timeVal = r.Time
					break
				}
			}
			row = append(row, timeVal)
		}
		if err := writer.Write(row); err != nil {
			return err
		}
	}

	fmt.Printf("Benchmark report written to %s\n", reportFile)
	return nil
}
