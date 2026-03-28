// #region 🔖Header
// 💻 semio/go/cmd/nativebridge/main.go
// Specs: Read JSON op+payload from stdin; write JSON result for engine native-algorithms REST.
// Summary: CLI bridge invoking semio.FlattenDesign and semio.DeletePiecesAndConnectionsInDesign.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

package main

import (
	"encoding/json"
	"io"
	"os"

	semio "github.com/usalu/semio/go/semio"
)

type bridgeRequest struct {
	Op              string          `json:"op"`
	Kit             json.RawMessage `json:"kit"`
	Design          json.RawMessage `json:"design"`
	DesignGuid      string          `json:"designGuid"`
	PieceGuids      []string        `json:"pieceGuids"`
	ConnectionGuids []string        `json:"connectionGuids"`
}

type bridgeResponse struct {
	Ok     bool            `json:"ok"`
	Result json.RawMessage `json:"result,omitempty"`
	Error  string          `json:"error,omitempty"`
}

func main() {
	body, err := io.ReadAll(os.Stdin)
	if err != nil {
		writeErr("read stdin: " + err.Error())
		return
	}
	var req bridgeRequest
	if err := json.Unmarshal(body, &req); err != nil {
		writeErr("parse request: " + err.Error())
		return
	}
	var kit semio.Kit
	if err := json.Unmarshal(req.Kit, &kit); err != nil {
		writeErr("parse kit: " + err.Error())
		return
	}
	switch req.Op {
	case "flatten":
		diff := semio.FlattenDesign(&kit, req.DesignGuid)
		out, err := json.Marshal(diff)
		if err != nil {
			writeErr("marshal flatten: " + err.Error())
			return
		}
		writeOk(out)
	case "delete":
		var design semio.Design
		if err := json.Unmarshal(req.Design, &design); err != nil {
			writeErr("parse design: " + err.Error())
			return
		}
		diff := semio.DeletePiecesAndConnectionsInDesign(&kit, design, req.PieceGuids, req.ConnectionGuids)
		out, err := json.Marshal(diff)
		if err != nil {
			writeErr("marshal delete: " + err.Error())
			return
		}
		writeOk(out)
	default:
		writeErr("unknown op: " + req.Op)
	}
}

func writeOk(result json.RawMessage) {
	_ = json.NewEncoder(os.Stdout).Encode(bridgeResponse{Ok: true, Result: result})
}

func writeErr(msg string) {
	_ = json.NewEncoder(os.Stdout).Encode(bridgeResponse{Ok: false, Error: msg})
}
