// #region 🔖Header
// [🧰semiorepo📚go💻emitgo](semiorepo://file/semio-repo/go/emit.go)
// 2025 Ueli Saluz <ueli@semio-tech.com>
// GPL-3.0
// Client helper to POST events to the semio-repo server.
// #region 🔖License

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🔖License

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
// [🧰semiorepo📚go💻emitgo🛠️emit](semiorepo://definition/semio-repo/go/emit.go/Emit)
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
