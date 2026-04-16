// #region Kit graph session (TypeScript parity: commitKitGraphChange, backbone, transactions, history)

package semio

import (
	"encoding/json"
	"fmt"
)

// Backbone receives outbound graph commits; implementations may run Changed asynchronously.
type Backbone interface {
	Changed(KitGraphChange) error
}

// BackboneAttach adds inbound diff delivery (optional).
type BackboneAttach interface {
	Backbone
	Attach(kit *Kit, onInbound func(KitDiff)) error
}

// KitGraphChange bundles forward/backward diffs with validation (TypeScript KitChange).
type KitGraphChange struct {
	Forward    KitDiff
	Backward   KitDiff
	Validation KitDiffValidationResult
}

// KitCommitOptions configures CommitKitGraphChange.
type KitCommitOptions struct {
	Origin            string
	TransactionID     string
	NotifyBackbone    *bool
	SkipGlobalHistory bool
}

type kitOpenTransaction struct {
	startKit Kit
	steps    []KitGraphChange
	redo     []KitGraphChange
}

func (k *Kit) ensureGraphMaps() {
	if k.openTransactions == nil {
		k.openTransactions = make(map[string]*kitOpenTransaction)
	}
	if k.flattenMerkle == nil {
		k.flattenMerkle = make(map[string]map[string]FlatMerkleCacheEntry)
	}
}

// kitStripRuntime copies domain fields only (no mutex / session), for JSON snapshot.
func kitStripRuntime(k *Kit) Kit {
	if k == nil {
		return Kit{}
	}
	return Kit{
		Guid:        k.Guid,
		Name:        k.Name,
		Version:     k.Version,
		Types:       k.Types,
		Designs:     k.Designs,
		Tags:        k.Tags,
		Concepts:    k.Concepts,
		Families:    k.Families,
		Qualities:   k.Qualities,
		Files:       k.Files,
		Folders:     k.Folders,
		Authors:     k.Authors,
		Remote:      k.Remote,
		Homepage:    k.Homepage,
		License:     k.License,
		Preview:     k.Preview,
		Icon:        k.Icon,
		Image:       k.Image,
		Description: k.Description,
		Attributes:  k.Attributes,
		CreatedAt:   k.CreatedAt,
		UpdatedAt:   k.UpdatedAt,
	}
}

func kitSnapshotKit(k *Kit) (Kit, error) {
	if k == nil {
		return Kit{}, fmt.Errorf("nil kit")
	}
	slim := kitStripRuntime(k)
	data, err := json.Marshal(slim)
	if err != nil {
		return Kit{}, err
	}
	var out Kit
	if err := json.Unmarshal(data, &out); err != nil {
		return Kit{}, err
	}
	return out, nil
}

// CommitKitGraphChange validates, inverts, applies, records history/transaction, notifies backbone (see KitCommitOptions).
func CommitKitGraphChange(kit *Kit, diff KitDiff, opts *KitCommitOptions) (KitGraphChange, error) {
	if kit == nil {
		return KitGraphChange{}, fmt.Errorf("nil kit")
	}
	o := KitCommitOptions{}
	if opts != nil {
		o = *opts
	}
	return kit.commitGraphChange(diff, o)
}

// SetBackbone attaches a backbone and optionally runs Attach for inbound diffs.
func (k *Kit) SetBackbone(b Backbone) {
	k.graphMu.Lock()
	k.backbone = b
	k.graphMu.Unlock()
	if b == nil {
		return
	}
	if att, ok := b.(BackboneAttach); ok {
		go func() {
			_ = att.Attach(k, func(in KitDiff) {
				_, _ = CommitKitGraphChange(k, in, &KitCommitOptions{NotifyBackbone: KitNotifyDisable()})
			})
		}()
	}
}

func ptrBool(v bool) *bool { return &v }

// KitNotifyDisable returns a *bool false for KitCommitOptions.NotifyBackbone.
func KitNotifyDisable() *bool { f := false; return &f }

// SetStrictMode when true treats validation warnings like errors.
func (k *Kit) SetStrictMode(strict bool) {
	k.graphMu.Lock()
	k.strictMode = strict
	k.graphMu.Unlock()
}

// ClearConflict clears the conflict lock without mutating entities.
func (k *Kit) ClearConflict() {
	k.graphMu.Lock()
	k.conflicted = false
	k.conflictErrors = nil
	k.conflictWarnings = nil
	k.graphMu.Unlock()
}

func (k *Kit) IsConflict() bool {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	return k.conflicted
}

func (k *Kit) ValidationSnapshot() KitDiffValidationResult {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	return KitDiffValidationResult{
		Ok:       !k.conflicted,
		Errors:   append([]KitDiffValidationNote(nil), k.conflictErrors...),
		Warnings: append([]KitDiffValidationNote(nil), k.conflictWarnings...),
	}
}

// StartTransaction opens a new transaction; multiple may be open. Pass ID to CommitKitGraphChange.
func (k *Kit) StartTransaction() (string, error) {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	if k.conflicted {
		return "", fmt.Errorf("kit conflicted; call ClearConflict first")
	}
	k.ensureGraphMaps()
	start, err := kitSnapshotKit(k)
	if err != nil {
		return "", err
	}
	id := Guid()
	k.openTransactions[id] = &kitOpenTransaction{startKit: start, steps: nil, redo: nil}
	return id, nil
}

func (k *Kit) AbortTransaction(txID string) error {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	if k.conflicted {
		return fmt.Errorf("kit conflicted")
	}
	tx, ok := k.openTransactions[txID]
	if !ok {
		return fmt.Errorf("unknown transaction %q", txID)
	}
	for i := len(tx.steps) - 1; i >= 0; i-- {
		ApplyKitDiff(k, &tx.steps[i].Backward)
	}
	delete(k.openTransactions, txID)
	k.conflicted = false
	k.conflictErrors = nil
	k.conflictWarnings = nil
	return nil
}

// FinalizeTransaction squashes net diff vs start snapshot, validates, pushes global history, notifies backbone once.
func (k *Kit) FinalizeTransaction(txID string) (KitGraphChange, error) {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	if k.conflicted {
		return KitGraphChange{}, fmt.Errorf("kit conflicted")
	}
	tx, ok := k.openTransactions[txID]
	if !ok {
		return KitGraphChange{}, fmt.Errorf("unknown transaction %q", txID)
	}

	forward := GetKitDiff(tx.startKit, *k)
	val := ValidateKitDiff(tx.startKit, forward, false)
	if !val.Ok || len(val.Errors) > 0 {
		return KitGraphChange{}, fmt.Errorf("finalize validation failed: %v", val.Errors)
	}
	if k.strictMode && len(val.Warnings) > 0 {
		return KitGraphChange{}, fmt.Errorf("finalize warnings (strict): %v", val.Warnings)
	}
	diffToApply := forward
	if val.Diff != nil {
		diffToApply = *val.Diff
	}
	backward := InverseKitDiff(tx.startKit, diffToApply)
	squashed := KitGraphChange{Forward: diffToApply, Backward: backward, Validation: val}

	delete(k.openTransactions, txID)
	k.historyPast = append(k.historyPast, squashed)
	k.historyFuture = nil

	if k.backbone != nil {
		ch := squashed
		go func() { _ = k.backbone.Changed(ch) }()
	}
	return squashed, nil
}

func (k *Kit) UndoWithinTransaction(txID string) {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	tx, ok := k.openTransactions[txID]
	if !ok || len(tx.steps) == 0 {
		return
	}
	ch := tx.steps[len(tx.steps)-1]
	tx.steps = tx.steps[:len(tx.steps)-1]
	ApplyKitDiff(k, &ch.Backward)
	tx.redo = append(tx.redo, ch)
}

func (k *Kit) RedoWithinTransaction(txID string) {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	tx, ok := k.openTransactions[txID]
	if !ok || len(tx.redo) == 0 {
		return
	}
	ch := tx.redo[len(tx.redo)-1]
	tx.redo = tx.redo[:len(tx.redo)-1]
	ApplyKitDiff(k, &ch.Forward)
	tx.steps = append(tx.steps, ch)
}

func (k *Kit) CanUndoWithinTransaction(txID string) bool {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	tx, ok := k.openTransactions[txID]
	return ok && len(tx.steps) > 0
}

func (k *Kit) CanRedoWithinTransaction(txID string) bool {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	tx, ok := k.openTransactions[txID]
	return ok && len(tx.redo) > 0
}

func (k *Kit) UndoHistory() error {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	if k.conflicted {
		return fmt.Errorf("kit conflicted")
	}
	if len(k.historyPast) == 0 {
		return nil
	}
	ch := k.historyPast[len(k.historyPast)-1]
	k.historyPast = k.historyPast[:len(k.historyPast)-1]
	ApplyKitDiff(k, &ch.Backward)
	k.historyFuture = append(k.historyFuture, ch)
	return nil
}

func (k *Kit) RedoHistory() error {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	if k.conflicted {
		return fmt.Errorf("kit conflicted")
	}
	if len(k.historyFuture) == 0 {
		return nil
	}
	ch := k.historyFuture[len(k.historyFuture)-1]
	k.historyFuture = k.historyFuture[:len(k.historyFuture)-1]
	ApplyKitDiff(k, &ch.Forward)
	k.historyPast = append(k.historyPast, ch)
	return nil
}

func (k *Kit) CanUndoHistory() bool {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	return len(k.historyPast) > 0
}

func (k *Kit) CanRedoHistory() bool {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	return len(k.historyFuture) > 0
}

// TransactFinalized runs fn with a transaction id, then finalizes or aborts on error.
func (k *Kit) TransactFinalized(fn func(txID string) error) error {
	id, err := k.StartTransaction()
	if err != nil {
		return err
	}
	if err := fn(id); err != nil {
		_ = k.AbortTransaction(id)
		return err
	}
	_, err = k.FinalizeTransaction(id)
	return err
}

func (k *Kit) commitGraphChange(diff KitDiff, o KitCommitOptions) (KitGraphChange, error) {
	k.graphMu.Lock()
	defer k.graphMu.Unlock()
	if k.conflicted {
		return KitGraphChange{}, fmt.Errorf("kit conflicted; call ClearConflict first")
	}
	val := ValidateKitDiff(*k, diff, false)
	if !val.Ok || len(val.Errors) > 0 {
		k.conflicted = true
		k.conflictErrors = val.Errors
		k.conflictWarnings = val.Warnings
		return KitGraphChange{}, fmt.Errorf("validation failed: %v", val.Errors)
	}
	if k.strictMode && len(val.Warnings) > 0 {
		k.conflicted = true
		k.conflictErrors = val.Errors
		k.conflictWarnings = val.Warnings
		return KitGraphChange{}, fmt.Errorf("validation warnings (strict): %v", val.Warnings)
	}
	diffToApply := diff
	if val.Diff != nil {
		diffToApply = *val.Diff
	}
	backward := InverseKitDiff(*k, diffToApply)
	ch := KitGraphChange{Forward: diffToApply, Backward: backward, Validation: val}
	ApplyKitDiff(k, &diffToApply)

	k.ensureGraphMaps()
	if o.TransactionID != "" {
		tx, ok := k.openTransactions[o.TransactionID]
		if !ok {
			return KitGraphChange{}, fmt.Errorf("unknown transaction %q", o.TransactionID)
		}
		tx.steps = append(tx.steps, ch)
		tx.redo = nil
	} else if !o.SkipGlobalHistory {
		k.historyPast = append(k.historyPast, ch)
		k.historyFuture = nil
	}

	notify := o.TransactionID == ""
	if o.NotifyBackbone != nil {
		notify = *o.NotifyBackbone
	}
	if o.TransactionID != "" {
		notify = false
	}
	if notify && k.backbone != nil {
		b := k.backbone
		c := ch
		go func() { _ = b.Changed(c) }()
	}

	k.conflicted = false
	k.conflictErrors = nil
	k.conflictWarnings = nil
	return ch, nil
}

// #endregion
