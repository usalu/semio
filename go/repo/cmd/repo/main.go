// #region Header
// SPDX-License-Identifier: AGPL-3.0-or-later
// #endregion Header

// #region Package
package main

// #endregion Package

// #region Imports
import (
	"context"
	"os"

	repo "github.com/usalu/semio/go/repo"
	"github.com/usalu/semio/go/repo/internal/adapters/cli"
	"github.com/usalu/semio/go/repo/internal/core"
)

// #endregion Imports

// #region Types
type graphqlAdapter struct {
	exec *repo.Executor
}

// #endregion Types

// #region GraphQL
func (g graphqlAdapter) Execute(ctx context.Context, query string, variables map[string]interface{}) (interface{}, error) {
	return g.exec.Execute(ctx, query, variables)
}

// #endregion GraphQL

// #region Engine
func buildEngine(config cli.Config) (*core.Engine, error) {
	repoRoot := config.Repo
	if repoRoot == "" {
		repoRoot = repo.GetRootDir()
	}
	repo.SetRootDir(repoRoot)
	executor, err := repo.NewExecutorWithContext(repoRoot, repo.NewRepoContext(repoRoot))
	if err != nil {
		return nil, err
	}
	return core.NewEngine(graphqlAdapter{exec: executor}), nil
}

// #endregion Engine

// #region Main
func main() {
	err := cli.Execute(buildEngine)
	if err == nil {
		return
	}
	if exitErr, ok := err.(cli.ExitError); ok {
		if exitErr.Code != 0 {
			os.Exit(exitErr.Code)
		}
		return
	}
	os.Exit(1)
}

// #endregion Main
