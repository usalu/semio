#!/usr/bin/env python3
"""🗃️ In-memory workspace over the repository: reads from disk lazily, buffers edits, and flushes only files whose on-disk content is still the one that was read."""
import os, hashlib
from pathlib import Path

REPO = '/Users/ueli/Documents/semio'


class WS:
    def __init__(self, repo=REPO):
        self.repo = repo
        self.orig = {}
        self.cur = {}
        self.deleted = set()
        self.created = set()

    def p(self, path):
        path = str(path)
        return path if path.startswith('/') else os.path.join(self.repo, path)

    def read(self, path):
        path = self.p(path)
        if path in self.deleted:
            raise FileNotFoundError(path)
        if path in self.cur:
            return self.cur[path]
        if path not in self.orig:
            self.orig[path] = Path(path).read_text()
        return self.orig[path]

    def exists(self, path):
        path = self.p(path)
        if path in self.deleted:
            return False
        return path in self.cur or Path(path).exists()

    def write(self, path, text):
        path = self.p(path)
        if path not in self.orig and path not in self.cur and Path(path).exists():
            self.orig[path] = Path(path).read_text()
        if path not in self.orig and not Path(path).exists():
            self.created.add(path)
        self.deleted.discard(path)
        self.cur[path] = text

    def delete(self, path):
        path = self.p(path)
        if path not in self.orig and Path(path).exists():
            self.orig[path] = Path(path).read_text()
        self.cur.pop(path, None)
        self.deleted.add(path)

    def changed(self):
        out = []
        for path, text in self.cur.items():
            if path in self.created or self.orig.get(path) != text:
                out.append(path)
        out += [p for p in self.deleted]
        return sorted(set(out))

    def flush(self, paths=None, dry=False):
        problems = []
        todo = self.changed() if paths is None else [self.p(x) for x in paths]
        for path in todo:
            if path in self.orig and Path(path).exists() and Path(path).read_text() != self.orig[path]:
                problems.append(path)
        if problems:
            raise RuntimeError('concurrent edit detected on: ' + '; '.join(problems))
        if dry:
            return todo
        for path in todo:
            if path in self.deleted:
                if Path(path).exists():
                    Path(path).unlink()
                continue
            Path(path).parent.mkdir(parents=True, exist_ok=True)
            Path(path).write_text(self.cur[path])
            self.orig[path] = self.cur[path]
            self.created.discard(path)
        return todo
