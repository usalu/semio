// #region 🔖Header
// [🧰semiorepo📚go💻emit](semiorepo://p/i/semio-repo/b/l/go/f/emit.go)
// 2025 Ueli Saluz <ueli@semio-tech.com>
// GPL-3.0
// Client helper to POST events to the semio-repo server.
// #endregion 🔖Header

package repo

import (
	"bytes"
	"encoding/json"
	"net/http"
	"os"
	"strings"
	"time"
)

// Emit posts an event to the semio-repo server. No-op when SEMIO_SERVER_ADDR is unset.
// Emit MUST perform the Emit operation.
// [🧰semiorepo📚go💻emit🛠️emit](semiorepo://p/i/semio-repo/b/l/go/f/emit.go/d/i/Emit)
func Emit(kind EventKind, source string, payload interface{}) {
	addr := strings.TrimSpace(os.Getenv("SEMIO_SERVER_ADDR"))
	if addr == "" {
		return
	}
	payloadBytes, err := json.Marshal(payload)
	if err != nil {
		return
	}
	ev := Event{Kind: kind, Source: source, Payload: payloadBytes}
	body, err := json.Marshal(ev)
	if err != nil {
		return
	}
	url := addr
	if !strings.HasPrefix(addr, "http://") && !strings.HasPrefix(addr, "https://") {
		url = "http://" + addr
	}
	url = strings.TrimSuffix(url, "/") + "/events"
	req, err := http.NewRequest(http.MethodPost, url, bytes.NewReader(body))
	if err != nil {
		return
	}
	req.Header.Set("Content-Type", "application/json")
	if token := strings.TrimSpace(os.Getenv("SEMIO_SERVER_TOKEN")); token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}
	client := &http.Client{Timeout: 5 * time.Second}
	_, _ = client.Do(req)
}
