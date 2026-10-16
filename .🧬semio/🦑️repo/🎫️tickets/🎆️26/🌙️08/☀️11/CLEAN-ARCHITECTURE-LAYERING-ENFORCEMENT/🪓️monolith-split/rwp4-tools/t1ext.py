# 🌐️ WP4 T1 external call sites: `<recv>.shell`-style receivers of ShellState in tests and in other packages' files, rewritten onto the components.
import re, sys, os, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from t1lib import *
from t1rewrite import *

def shell_vars(raw):
    b = blank(raw)
    helpers = set(re.findall(r'fn\s+([A-Za-z_]\w*)\s*(?:<[^>]*>)?\s*\([^)]*\)\s*->\s*(?:[A-Za-z_]\w*::)*ShellState\b', b))
    names = {'shell'}
    hp = '|'.join(sorted(helpers | {'ShellState::new'}, key=len, reverse=True))
    for m in re.finditer(r'let\s+(?:mut\s+)?([A-Za-z_]\w*)\s*(?::[^=;]*)?=\s*(?:' + hp + r')\s*\(', b): names.add(m.group(1))
    for m in re.finditer(r'let\s+\(([^)]*)\)\s*=\s*\(((?:[^;]|\n)*?)\)\s*;', b):
        lhs = [x.strip().replace('mut ', '') for x in m.group(1).split(',')]
        rhs = re.split(r',\s*(?=(?:' + hp + r')\s*\(|[A-Za-z_])', m.group(2))
        for i, x in enumerate(lhs):
            if i < len(rhs) and re.match(r'\s*(?:' + hp + r')\s*\(', rhs[i]): names.add(x)
    for m in re.finditer(r'(?<![\w])(?<![\w)\]]\.)(?:mut\s+)?([A-Za-z_]\w*)\s*:\s*&\s*(?:mut\s+)?(?:[A-Za-z_]\w*::)*ShellState\b', b): names.add(m.group(1))
    for m in re.finditer(r'(?<![\w])(?<![\w)\]]\.)(?:mut\s+)?([A-Za-z_]\w*)\s*:\s*(?:[A-Za-z_]\w*::)*ShellState\b', b): names.add(m.group(1))
    return names

def recv_regex(names):
    alt = '|'.join(sorted(map(re.escape, names), key=len, reverse=True))
    return re.compile(r'(?<![\w])((?:[A-Za-z_]\w*(?:\(\))?\s*\.\s*)*(?:' + alt + r'))\s*\.\s*([A-Za-z_]\w*)')

RECV = recv_regex({'shell'})
FREE_ARG = {}
for n in FREE_TIER1:
    head = r'UiPrefsSnapshot::capture\(' if '::' in n else r'(?<![\w.:])' + re.escape(n) + r'\s*\('
    FREE_ARG[n] = re.compile(head + r'\s*(&\s*mut\s+|&\s*)?([A-Za-z_]\w*)\b')

PATH_REPOINTS = [
    ('crate::winit_app::dispatch_normalized_event(', 'crate::dispatch_normalized_event('),
    ('use crate::program_bridge::window_engagements_section_tests::{engagements_section_document, fixture as engagements_fixture};', 'use crate::shell::section_fixtures::{engagements_fixture, engagements_section_document};'),
    ('use crate::program_bridge::window_measures_section_tests::{fixture, measures_section_document};', 'use crate::shell::section_fixtures::{fixture, measures_section_document};'),
    ('crate::program_bridge::window_measures_section_tests::section_document(', 'crate::shell::section_fixtures::section_document('),
    ('use crate::canvas_presence::tests::{numeric, peer};', 'use crate::shell::section_fixtures::{numeric, peer};'),
]

def rewrite_external(raw, report, who='', recv_vars=('shell',)):
    b = blank(raw)
    edits = []
    vars_ = shell_vars(raw)
    for m in recv_regex(vars_).finditer(b):
        recv, x = m.group(1), m.group(2)
        after = m.end()
        k0 = m.start(1) - 1
        while k0 >= 0 and b[k0].isspace(): k0 -= 1
        chained = k0 >= 1 and b[k0] == '.' and b[k0 - 1] != '.'
        cm = CALL_AFTER.match(b, after)
        if cm:
            if x in TIER1 and PLAN1[x].get('recv') != 'free':
                cinfo = PLAN1[x]; H = cinfo['struct']
                edits.append((m.start(1), m.end(1), f'{recv}.{FIELD_OF[H]}'))
                lp = cm.end() - 1; rp = find_matching(b, lp)
                cex = [('&mut ' if mm else '&') + f'{recv}.{FIELD_OF[k]}' for k, mm in cinfo['params']]
                if cex and chained: report.append((who, f'chained receiver needs components for {x}'))
                if cex:
                    args = b[lp + 1:rp]
                    if args.strip() == '': edits.append((lp + 1, lp + 1, ', '.join(cex)))
                    else:
                        e = lp + 1 + len(args.rstrip())
                        edits.append((e, e, (' ' if b[e - 1] == ',' else ', ') + ', '.join(cex)))
        elif x in F2S:
            edits.append((m.start(1), m.end(1), f'{recv}.{FIELD_OF[F2S[x]]}'))
        elif x in TIER1:
            report.append((who, f'method {x} used as a value on {recv}'))
    for g, rx in FREE_ARG.items():
        for m in rx.finditer(b):
            params = PLAN1[g]['params']
            amp = m.group(1) or ''
            var = m.group(2)
            exprs = [('&mut ' if mm else '&') + f'{var}.{FIELD_OF[k]}' for k, mm in params]
            edits.append((m.start(1) if m.group(1) else m.start(2), m.end(2), ', '.join(exprs)))
    for m in re.finditer(r'(?<![\w])(?<![\w)\]]\.)ShellState::([A-Za-z_]\w*)\s*\(', b):
        x = m.group(1)
        if x in TIER1 and PLAN1[x].get('recv') != 'free':
            if PLAN1[x]['recv'] is None:
                if PLAN1[x]['struct'] is None: edits.append((m.start(), m.start() + len('ShellState::'), ''))
                else: edits.append((m.start(), m.start() + len('ShellState'), PLAN1[x]['struct']))
            else:
                report.append((who, f'ShellState::{x} UFCS call'))
    out = raw
    for s, e, t in sorted(set(edits), key=lambda x: (x[0], x[1]), reverse=True):
        out = out[:s] + t + out[e:]
    for a, b in PATH_REPOINTS:
        out = out.replace(a, b)
    return out
