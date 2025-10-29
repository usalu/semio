// #region Header

// hotkeys.tsx

// 2025 Ueli Saluz

// #endregion

import { useCallback, useEffect } from "react";
import { useHotkeys as useHotkeysHook } from "react-hotkeys-hook";
import { useTranslation } from "react-i18next";
import { useSketchpadStore } from "./store";

export type HotkeyPath = string;
export type HotkeyValue = string;
export type HotkeyOverrides = Record<HotkeyPath, HotkeyValue>;

export function getHotkeyPath(...parts: string[]): HotkeyPath {
  return parts.filter(Boolean).join(".");
}

export function useHotkey(path: HotkeyPath): HotkeyValue | undefined {
  const { t } = useTranslation();
  const store = useSketchpadStore();
  const overrides = store.snapshot().hotkeyOverrides || {};
  if (overrides[path]) return overrides[path];
  const fullPath = `${path}.hotkey`;
  const value = t(fullPath);
  if (value === fullPath) return undefined;
  return value;
}

export function useHotkeys(path: HotkeyPath, callback: () => void, deps: any[] = []): void {
  const hotkey = useHotkey(path);
  useHotkeysHook(hotkey || "", callback, { enabled: !!hotkey }, deps);
}

export function useSetHotkey(): (path: HotkeyPath, value: HotkeyValue) => void {
  const store = useSketchpadStore();
  return useCallback(
    (path: HotkeyPath, value: HotkeyValue) => {
      store.executeCommand("semio.sketchpad.setHotkey", path, value);
    },
    [store],
  );
}

export function useResetHotkey(): (path: HotkeyPath) => void {
  const store = useSketchpadStore();
  return useCallback(
    (path: HotkeyPath) => {
      store.executeCommand("semio.sketchpad.resetHotkey", path);
    },
    [store],
  );
}

export function useResetAllHotkeys(): () => void {
  const store = useSketchpadStore();
  return useCallback(() => {
    store.executeCommand("semio.sketchpad.resetAllHotkeys");
  }, [store]);
}

export function useHotkeyOverrides(): HotkeyOverrides {
  const store = useSketchpadStore();
  return store.snapshot().hotkeyOverrides || {};
}

export function useNavigateToHotkeySetting(): (path: HotkeyPath) => void {
  const store = useSketchpadStore();
  return useCallback(
    (path: HotkeyPath) => {
      store.executeCommand("semio.sketchpad.navigateToHotkeySetting", path);
    },
    [store],
  );
}
