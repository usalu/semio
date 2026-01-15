package main

import (
	"fmt"
	"os"
	"os/exec"

	"github.com/spf13/cobra"
)

var preflightCmd = &cobra.Command{
	Use:   "preflight [command]",
	Short: "Run preflight checks (fix, analyze, test, build, publish)",
	RunE:  runPreflight,
}

func init() {
	rootCmd.AddCommand(preflightCmd)
}

func runPreflight(cmd *cobra.Command, args []string) error {
	command := "preflight"
	if len(args) > 0 {
		command = args[0]
	}

	// skip := make(map[string]bool)
	// Parse skip flags manually since we might be passing args through
	// (Implementation simplification: relying on cobra flags would be better but keeping it simple)

	// TODO: Add flag parsing for --skip and --nx
	
	// For now, mapping directly to functionality
	switch command {
	case "fix":
		return runPreflightFix()
	case "analyze":
		return runPreflightAnalyze()
	case "preflight":
		if err := runPreflightFix(); err != nil {
			return err
		}
		return runPreflightAnalyze()
	case "test":
		if err := runPreflightFix(); err != nil {
			return err
		}
		if err := runPreflightAnalyze(); err != nil {
			return err
		}
		return runNx("test")
	case "build":
		// Implied test
		if err := runNx("test"); err != nil {
			return err
		}
		return runNx("build")
	case "publish:test":
		if err := runNx("build"); err != nil {
			return err
		}
		return runNx("publish:test")
	case "publish":
		if err := runNx("build"); err != nil {
			return err
		}
		return runNx("publish")
	default:
		return fmt.Errorf("unknown command: %s", command)
	}
}

func runPreflightFix() error {
	// Call the internal fix logic
	// In main.go, fixCmd logic is: mutation Fix($scope: String) ...
	// We can execute the same logic here or call the command.
	// Since we are in the same package, we can just call the handler or share logic.
	// But the handler expects cobra command/args.
	
	// For now, we will just print what we are doing, but strictly we should invoke the internal fix.
	// The existing preflight.ts called "hooks/code.tsx --fix" etc.
	// The new design seems to center around `repo fix`.
	// So we will invoke `repo fix`.
	
	fmt.Println("Running fix...")
	// We can reuse the fixCmd RunE if we construct a dummy command, or better, refactor main.go to expose the logic.
	// For now, let's assume we want to run the global fix.
	// We'll call the function that fixCmd calls.
	
	// But fixCmd uses graphql. Let's just run the fix command logic directly.
	// Ideally we would refactor `main.go` to separate logic from CLI.
	// Given I can't easily refactor main.go massively right now without risk, I will use the `fixCmd` variable.
	
	return fixCmd.RunE(fixCmd, []string{})
}

func runPreflightAnalyze() error {
	fmt.Println("Running analyze...")
	return analyzeCmd.RunE(analyzeCmd, []string{})
}

func runNx(target string, args ...string) error {
	fmt.Printf("Running nx %s...\n", target)
	cmdArgs := []string{"nx", "run-many", "-t", target}
	cmdArgs = append(cmdArgs, args...)
	
	cmd := exec.Command("npx", cmdArgs...)
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	
	return cmd.Run()
}
