// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🏁️benchmark carries the benchmark command of the cli.

// #endregion 🧲️Header

package cli

import (
	fmt "fmt"
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	sync "sync"

	metricspkg "github.com/usalu/semio/repo/metrics"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🔮️Benchmark Command

// 💿️benchmarkCmd holds the data fields for a benchmarkCmd record.
var benchmarkCmd = &Command{
	Use:   "benchmark",
	Short: "Run benchmarks for all ecosystems",
	RunE:  runBenchmark,
}

// 🔷️benchmarkDryRun holds the data fields for a benchmarkDryRun record.
var benchmarkDryRun bool

// 🔹️runBenchmark holds the data fields for a runBenchmark record.
func runBenchmark(cmd *Command, args []string) error {
	if benchmarkDryRun {
		return nil
	}
	rootDir := workspace.FindRepoRoot(".")
	results := make([]metricspkg.BenchmarkResult, 0)
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
			Args:    []string{"tsx", "compose.benchmark.ts"},
			Dir:     filepath.Join(rootDir, "js", "compose"),
			Enabled: true,
		},
		{
			Name:    "Python",
			Cmd:     "uv",
			Args:    []string{"run", "compose.benchmark.py"},
			Dir:     filepath.Join(rootDir, "py", "compose"),
			Enabled: true,
		},
		{
			Name:    "Go",
			Cmd:     "go",
			Args:    []string{"run", "compose_benchmark.go"},
			Dir:     filepath.Join(rootDir, "go", "compose"),
			Enabled: true,
		},
		{
			Name:    "C#",
			Cmd:     "dotnet",
			Args:    []string{"run", "--technology", "Compose.Benchmark/Compose.Benchmark.csproj", "--configuration", "Release"},
			Dir:     filepath.Join(rootDir, "net"),
			Enabled: true,
		},
		{
			Name:    "Rust",
			Cmd:     "cargo",
			Args:    []string{"run", "--release", "--bin", "compose-benchmark"},
			Dir:     filepath.Join(rootDir, "rs", "compose"),
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
			if _, err := os.Stat(t.Dir); os.IsNotExist(err) {
				fmt.Printf("Skipping %s: directory %s not found\n", t.Name, t.Dir)
				return
			}

			c := workspace.ToolCommand(t.Cmd, t.Args...)
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
			results = append(results, metricspkg.ParseBenchmarkOutput(t.Name, string(output))...)
			mu.Unlock()
		}(task)
	}

	wg.Wait()
	if len(results) > 0 {
		return writeBenchmarkReport(rootDir, results)
	}

	return nil
}

// ✏️writeBenchmarkReport writes the 📊️metrics benchmark table to the repository metrics directory.
func writeBenchmarkReport(rootDir string, results []metricspkg.BenchmarkResult) error {
	_ = rootDir
	reportFile := filepath.Join(workspace.GetRepoMetaDir(), "📊️metrics", "benchmark.csv")
	if err := os.MkdirAll(filepath.Dir(reportFile), 0755); err != nil {
		return err
	}
	if err := os.WriteFile(reportFile, []byte(metricspkg.BenchmarkCSV(results)), 0644); err != nil {
		return err
	}
	fmt.Printf("Benchmark report written to %s\n", reportFile)
	return nil
}

// #endregion 🔮️Benchmark Command
