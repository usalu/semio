#!/usr/bin/env python3
"""Lexical masks: for each char of a .rs/.ts/.tsx source, 1 when inside a string, template text, regex literal or comment."""
import re


def mask_ts(s):
    n = len(s)
    m = bytearray(n)
    REGEX_PREV = set('(,=:[!&|?{};+-*%<>~^')
    KW = ('return', 'typeof', 'case', 'in', 'of', 'delete', 'void', 'throw', 'new', 'else', 'do', 'await', 'yield')

    def code(i, stop_brace):
        depth = 0
        last = ''
        while i < n:
            c = s[i]
            if c == '/' and s[i:i + 2] == '//':
                j = s.find('\n', i)
                j = n if j < 0 else j
                m[i:j] = b'\x01' * (j - i)
                i = j
                continue
            if c == '/' and s[i:i + 2] == '/*':
                j = s.find('*/', i + 2)
                j = n if j < 0 else j + 2
                m[i:j] = b'\x01' * (j - i)
                i = j
                continue
            if c in '"\'':
                j = i + 1
                while j < n and s[j] != c:
                    if s[j] == '\\':
                        j += 1
                    j += 1
                j = min(j + 1, n)
                m[i:j] = b'\x01' * (j - i)
                i = j
                last = 'x'
                continue
            if c == '`':
                i = template(i)
                last = 'x'
                continue
            if c == '/':
                prev_ok = last == '' or last in REGEX_PREV
                if not prev_ok:
                    k = i - 1
                    while k >= 0 and s[k] in ' \t\r\n':
                        k -= 1
                    w = re.search(r'([A-Za-z_]+)$', s[max(0, k - 12):k + 1])
                    if w and w.group(1) in KW:
                        prev_ok = True
                if prev_ok:
                    j = i + 1
                    incls = False
                    while j < n:
                        d = s[j]
                        if d == '\\':
                            j += 2
                            continue
                        if d == '[':
                            incls = True
                        elif d == ']':
                            incls = False
                        elif d == '/' and not incls:
                            break
                        elif d == '\n':
                            break
                        j += 1
                    j = min(j + 1, n)
                    while j < n and s[j].isalpha():
                        j += 1
                    m[i:j] = b'\x01' * (j - i)
                    i = j
                    last = 'x'
                    continue
            if c == '{':
                depth += 1
            elif c == '}':
                if stop_brace and depth == 0:
                    return i
                depth -= 1
            if not c.isspace():
                last = c
            i += 1
        return i

    def template(i):
        m[i] = 1
        j = i + 1
        while j < n:
            c = s[j]
            if c == '\\':
                m[j:j + 2] = b'\x01\x01'
                j += 2
                continue
            if c == '`':
                m[j] = 1
                return j + 1
            if c == '$' and s[j + 1:j + 2] == '{':
                m[j:j + 2] = b'\x01\x01'
                j = code(j + 2, True)
                if j < n:
                    m[j] = 1
                    j += 1
                continue
            m[j] = 1
            j += 1
        return j

    code(0, False)
    return m


def mask_rs(s):
    n = len(s)
    m = bytearray(n)
    i = 0
    while i < n:
        c = s[i]
        if c == '/' and s[i:i + 2] == '//':
            j = s.find('\n', i)
            j = n if j < 0 else j
            m[i:j] = b'\x01' * (j - i)
            i = j
            continue
        if c == '/' and s[i:i + 2] == '/*':
            depth = 1
            j = i + 2
            while j < n and depth:
                if s[j:j + 2] == '/*':
                    depth += 1
                    j += 2
                elif s[j:j + 2] == '*/':
                    depth -= 1
                    j += 2
                else:
                    j += 1
            m[i:j] = b'\x01' * (j - i)
            i = j
            continue
        prev_ident = i > 0 and (s[i - 1].isalnum() or s[i - 1] == '_')
        if c in 'rb' and not prev_ident:
            mm = re.match(r'(?:br|r)(#*)"', s[i:i + 40])
            if mm:
                h = mm.group(1)
                j = s.find('"' + h, i + mm.end())
                j = n if j < 0 else j + 1 + len(h)
                m[i:j] = b'\x01' * (j - i)
                i = j
                continue
        if c == '"' or (c == 'b' and s[i:i + 2] == 'b"' and not prev_ident):
            j = i + (2 if c == 'b' else 1)
            while j < n and s[j] != '"':
                if s[j] == '\\':
                    j += 1
                j += 1
            j = min(j + 1, n)
            m[i:j] = b'\x01' * (j - i)
            i = j
            continue
        if c == "'":
            mm = re.match(r"'(?:\\u\{[0-9a-fA-F_]+\}|\\x[0-9a-fA-F]{2}|\\.|[^\\'\n])'", s[i:i + 14])
            if mm:
                m[i:i + mm.end()] = b'\x01' * mm.end()
                i += mm.end()
                continue
        i += 1
    return m
