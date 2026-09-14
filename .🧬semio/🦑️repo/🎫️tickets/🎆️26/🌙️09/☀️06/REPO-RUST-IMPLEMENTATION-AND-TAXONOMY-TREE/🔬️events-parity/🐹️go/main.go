// 🐹️ Ticket-local parity probe: writes the Go implementation's own artefacts for the 📡️events
// module so they can be diffed byte for byte against the Rust twin and the Node oracle.
package main

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"

	events "github.com/usalu/semio/repo/events"
)

type storeVectors struct {
	Inputs []events.Input `json:"inputs"`
}

type exportVectors struct {
	Entities []struct {
		Kind  string            `json:"kind"`
		ID    string            `json:"id"`
		Value map[string]string `json:"value"`
	} `json:"entities"`
}

type payloadVectors struct {
	Cases []struct {
		ID    string          `json:"id"`
		Type  string          `json:"type"`
		Input json.RawMessage `json:"input"`
	} `json:"cases"`
}

func newPayload(name string) any {
	switch name {
	case "TicketPayload":
		return &events.TicketPayload{}
	case "TicketOpenPayload":
		return &events.TicketOpenPayload{}
	case "TicketClosePayload":
		return &events.TicketClosePayload{}
	case "TicketReopenPayload":
		return &events.TicketReopenPayload{}
	case "TicketChangePayload":
		return &events.TicketChangePayload{}
	case "GoalPayload":
		return &events.GoalPayload{}
	case "GoalOpenPayload":
		return &events.GoalOpenPayload{}
	case "GoalClosePayload":
		return &events.GoalClosePayload{}
	case "GoalReopenPayload":
		return &events.GoalReopenPayload{}
	case "GoalChangePayload":
		return &events.GoalChangePayload{}
	case "ContributorPayload":
		return &events.ContributorPayload{}
	case "CheckpointPayload":
		return &events.CheckpointPayload{}
	case "TodoPayload":
		return &events.TodoPayload{}
	case "TodoCreatePayload":
		return &events.TodoCreatePayload{}
	case "TodoChangePayload":
		return &events.TodoChangePayload{}
	case "TodoDeletePayload":
		return &events.TodoDeletePayload{}
	case "WorkItem":
		return &events.WorkItem{}
	case "ContributorWork":
		return &events.ContributorWork{}
	case "DraftPayload":
		return &events.DraftPayload{}
	case "FilePayload":
		return &events.FilePayload{}
	case "FolderPayload":
		return &events.FolderPayload{}
	case "SectionPayload":
		return &events.SectionPayload{}
	case "IntegratePayload":
		return &events.IntegratePayload{}
	case "ExtractPayload":
		return &events.ExtractPayload{}
	}
	return nil
}

func read(fixtures string, name string, target any) {
	data, err := os.ReadFile(filepath.Join(fixtures, name))
	if err != nil {
		panic(err)
	}
	if err := json.Unmarshal(data, target); err != nil {
		panic(err)
	}
}

func main() {
	fixtures := os.Args[1]
	out := os.Args[2]
	if err := os.MkdirAll(out, 0o755); err != nil {
		panic(err)
	}

	var store storeVectors
	read(fixtures, "🗄️store-vectors.json", &store)
	logPath := filepath.Join(out, "🐹️store.jsonl")
	_ = os.Remove(logPath)
	_ = os.Remove(logPath + ".stage")
	if _, err := (events.Store{Path: logPath}).Append(context.Background(), store.Inputs, nil); err != nil {
		panic(err)
	}

	var payloads payloadVectors
	read(fixtures, "✉️payload-vectors.json", &payloads)
	encodings := map[string]string{}
	for _, testCase := range payloads.Cases {
		value := newPayload(testCase.Type)
		if value == nil {
			panic("unknown payload type " + testCase.Type)
		}
		if err := json.Unmarshal(testCase.Input, value); err != nil {
			panic(err)
		}
		encoded, err := json.Marshal(value)
		if err != nil {
			panic(err)
		}
		encodings[testCase.ID] = string(encoded)
	}

	var export exportVectors
	read(fixtures, "📤️export-vectors.json", &export)
	entities := make([]events.ExportEntity, 0, len(export.Entities))
	for _, entity := range export.Entities {
		entities = append(entities, events.ExportEntity{Kind: entity.Kind, ID: entity.ID, Value: entity.Value})
	}
	snapshot, err := events.BuildExportSnapshot(context.Background(), entities)
	if err != nil {
		panic(err)
	}
	inputIds := make([]string, 0, len(snapshot.Inputs))
	for _, input := range snapshot.Inputs {
		inputIds = append(inputIds, input.ID)
	}

	kinds := make([]string, 0)
	for _, kind := range events.AllEventKinds() {
		kinds = append(kinds, string(kind))
	}

	envelope, err := json.Marshal(events.Event{Kind: events.EventTicketOpenStarting, Source: "repo-cli", Payload: json.RawMessage(`{"id":"a"}`)})
	if err != nil {
		panic(err)
	}

	report := map[string]any{
		"implementation": "go",
		"kinds":          kinds,
		"encodings":      encodings,
		"envelope":       string(envelope),
		"snapshot":       snapshot.Snapshot,
		"inputIds":       inputIds,
		"storeDigest":    "",
	}
	logBytes, err := os.ReadFile(logPath)
	if err != nil {
		panic(err)
	}
	report["storeDigest"] = events.Digest(logBytes)
	encoded, err := json.MarshalIndent(report, "", "  ")
	if err != nil {
		panic(err)
	}
	if err := os.WriteFile(filepath.Join(out, "🐹️report.json"), append(encoded, '\n'), 0o644); err != nil {
		panic(err)
	}
	fmt.Println("go report written")
}
