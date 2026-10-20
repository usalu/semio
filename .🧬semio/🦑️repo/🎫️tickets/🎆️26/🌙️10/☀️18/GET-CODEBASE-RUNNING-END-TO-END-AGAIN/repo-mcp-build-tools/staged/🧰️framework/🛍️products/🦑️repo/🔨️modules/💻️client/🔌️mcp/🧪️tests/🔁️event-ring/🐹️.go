// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Event ring tests: a session that outlives the event bounds keeps answering, and the retained window stays a verifiable hash chain.

// #endregion 🧲️Header

package main

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"testing"
)

// #region 🔁️EventRingTests

func ringInput(index int) EventInput {
	return EventInput{Kind: "probe", Peer: "p", Generation: 1, Payload: json.RawMessage(fmt.Sprintf(`{"n":%d}`, index))}
}

func TestRingForgetsTheOldestEventsAndKeepsAVerifiableChain(t *testing.T) {
	log := NewRingEventLog(1<<20, 10)
	for index := 1; index <= 35; index++ {
		if err := log.Commit(context.Background(), ringInput(index)); err != nil {
			t.Fatalf("event %d: %v", index, err)
		}
	}
	events := log.Events()
	if len(events) != 10 || events[0].Sequence != 26 || events[9].Sequence != 35 || events[0].Previous == "" {
		t.Fatalf("window %+v", events)
	}
	snapshot := log.Snapshot()
	replayed, err := ReplayEvents(context.Background(), snapshot, len(snapshot), 100)
	if err != nil || len(replayed) != 10 || replayed[0].Sequence != 26 {
		t.Fatalf("the retained window must replay: %v %d", err, len(replayed))
	}
	tampered := bytes.Replace(snapshot, []byte(`\"n\":30`), []byte(`\"n\":99`), 1)
	if bytes.Equal(tampered, snapshot) {
		tampered = bytes.Replace(snapshot, []byte(`"n":30`), []byte(`"n":99`), 1)
	}
	if _, err := ReplayEvents(context.Background(), tampered, len(tampered), 100); err == nil {
		t.Fatal("a changed event must break the chain")
	}
}

func TestRingBoundsBytesAsWellAsCount(t *testing.T) {
	log := NewRingEventLog(2048, 1000)
	for index := 1; index <= 200; index++ {
		if err := log.Commit(context.Background(), ringInput(index)); err != nil {
			t.Fatal(err)
		}
	}
	if len(log.Snapshot()) > 2048 || len(log.Events()) == 0 {
		t.Fatalf("bytes %d events %d", len(log.Snapshot()), len(log.Events()))
	}
	oversized := EventInput{Kind: "probe", Payload: json.RawMessage(`"` + string(bytes.Repeat([]byte("x"), 4096)) + `"`)}
	if err := log.Commit(context.Background(), oversized); !errors.Is(err, ErrLimit) {
		t.Fatalf("a single event beyond the bound is refused, not truncated: %v", err)
	}
}

func TestStrictLogStillRefusesAtItsBound(t *testing.T) {
	log := NewEventLog(1<<20, 3)
	for index := 1; index <= 3; index++ {
		if err := log.Commit(context.Background(), ringInput(index)); err != nil {
			t.Fatal(err)
		}
	}
	if err := log.Commit(context.Background(), ringInput(4)); !errors.Is(err, ErrLimit) {
		t.Fatalf("got %v", err)
	}
}

func TestProductionServerAnswersFarBeyondItsEventBound(t *testing.T) {
	limits := ProductionLimits()
	limits.MaxEvents = 40
	server, err := NewRepositoryServerWithLimits(&testRepository{}, limits)
	if err != nil {
		t.Fatal(err)
	}
	session, err := server.Connect("ring", nil)
	if err != nil {
		t.Fatal(err)
	}
	initializeSession(t, session)
	for index := 0; index < 500; index++ {
		response, err := session.Dispatch(context.Background(), request("ping", fmt.Sprintf(`"ping-%d"`, index), `{}`))
		if err != nil {
			t.Fatalf("ping %d ended the session: %v", index, err)
		}
		assertResponseCode(t, response, 0)
	}
	if len(server.Events().Events()) > 40 {
		t.Fatalf("events %d", len(server.Events().Events()))
	}
}

// #endregion 🔁️EventRingTests
