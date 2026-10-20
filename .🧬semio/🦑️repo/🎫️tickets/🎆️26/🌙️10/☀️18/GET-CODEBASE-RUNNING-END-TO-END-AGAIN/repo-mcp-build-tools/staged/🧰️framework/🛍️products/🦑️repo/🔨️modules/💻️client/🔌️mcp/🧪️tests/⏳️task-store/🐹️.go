// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Task store tests: lifetime, retention, capacity and ownership of tasks.

// #endregion 🧲️Header

package main

import (
	"errors"
	"testing"
	"time"
)

// #region ⏳️TaskStoreTests

func clockedStore(maximum int) (*TaskStore, *time.Time) {
	now := time.Date(2026, 10, 21, 8, 0, 0, 0, time.UTC)
	store := NewTaskStore(maximum)
	store.now = func() time.Time { return now }
	return store, &now
}

func TestFinishedTasksExpireAfterTheirTTLAndWorkingTasksNever(t *testing.T) {
	store, now := clockedStore(8)
	ttl := int64(1000)
	finished, _ := store.create(nil, &ttl)
	working, _ := store.create(nil, &ttl)
	store.finish(finished, &CallToolResult{}, nil)
	*now = now.Add(999 * time.Millisecond)
	if _, found := store.lookup(finished.task.TaskID); !found {
		t.Fatal("a task lives for its ttl")
	}
	*now = now.Add(2 * time.Millisecond)
	if _, found := store.lookup(finished.task.TaskID); found {
		t.Fatal("a finished task is forgotten after its ttl")
	}
	*now = now.Add(48 * time.Hour)
	if _, found := store.lookup(working.task.TaskID); !found {
		t.Fatal("a working task is never forgotten")
	}
}

func TestTTLIsBoundedAndDefaulted(t *testing.T) {
	store, _ := clockedStore(8)
	huge := MaximumTaskTTL * 10
	bounded, _ := store.create(nil, &huge)
	defaulted, _ := store.create(nil, nil)
	if *bounded.task.TTL != MaximumTaskTTL || *defaulted.task.TTL != DefaultTaskTTL {
		t.Fatalf("ttl %d %d", *bounded.task.TTL, *defaulted.task.TTL)
	}
}

func TestStoreForgetsTheOldestFinishedTaskWhenFullAndRefusesWhenAllWork(t *testing.T) {
	store, _ := clockedStore(2)
	first, _ := store.create(nil, nil)
	second, _ := store.create(nil, nil)
	if _, err := store.create(nil, nil); !errors.Is(err, ErrLimit) {
		t.Fatalf("a store full of working tasks refuses: %v", err)
	}
	store.finish(first, &CallToolResult{}, nil)
	third, err := store.create(nil, nil)
	if err != nil {
		t.Fatal(err)
	}
	if _, found := store.lookup(first.task.TaskID); found {
		t.Fatal("the oldest finished task makes room")
	}
	if _, found := store.lookup(second.task.TaskID); !found || third == nil {
		t.Fatal("working tasks stay")
	}
}

func TestFailureAndCancellationAreTerminalAndFinishIsOnce(t *testing.T) {
	store, _ := clockedStore(8)
	failed, _ := store.create(nil, nil)
	store.finish(failed, nil, &RPCError{Code: CodeInternalError, Message: "x"})
	if failed.task.Status != TaskFailed {
		t.Fatalf("status %s", failed.task.Status)
	}
	errorResult, _ := store.create(nil, nil)
	store.finish(errorResult, &CallToolResult{IsError: true}, nil)
	if errorResult.task.Status != TaskFailed {
		t.Fatalf("a tool error fails the task: %s", errorResult.task.Status)
	}
	cancelled, _ := store.create(nil, nil)
	stopped := false
	cancelled.cancel = func(error) { stopped = true }
	if !store.cancel(cancelled, errors.New("stop")) || !stopped || cancelled.task.Status != TaskCancelled {
		t.Fatal("cancel stops the call and ends the task")
	}
	if store.finish(cancelled, &CallToolResult{}, nil) || cancelled.task.Status != TaskCancelled {
		t.Fatal("a cancelled task keeps its state when the call returns later")
	}
	if store.cancel(cancelled, nil) {
		t.Fatal("a finished task cannot be cancelled")
	}
}

func TestClosingASessionCancelsItsWorkingTasksOnly(t *testing.T) {
	store, _ := clockedStore(8)
	owner, other := &Session{}, &Session{}
	mine, _ := store.create(owner, nil)
	theirs, _ := store.create(other, nil)
	store.CancelOwner(owner, errors.New("closed"))
	if mine.task.Status != TaskCancelled || theirs.task.Status != TaskWorking {
		t.Fatalf("%s %s", mine.task.Status, theirs.task.Status)
	}
}

// #endregion ⏳️TaskStoreTests
