#nullable enable

using System;

namespace Compose.Store;

/// <summary>📬 Kit command selection helpers — mirrors <c>compose/js/graphql-kit-selection.ts</c>.</summary>
internal static class StoreGraphqlSelection
{
    internal static string WithResponse(string kitSelection, string responseSelection)
    {
        var trimmed = kitSelection.Trim();
        var open = trimmed.IndexOf('{');
        if (open < 0) return AppendResponseAfterArgs(trimmed, responseSelection);
        var close = FindMatchingCloseBrace(trimmed, open);
        if (close < 0) return AppendResponseAfterArgs(trimmed, responseSelection);
        var head = trimmed[..open].TrimEnd();
        var inner = trimmed[(open + 1)..close].Trim();
        var tail = trimmed[(close + 1)..].Trim();
        var result = $"{head} {{ {TransformKitSelectionBlock(inner, responseSelection)} }}";
        return tail.Length == 0 ? result : $"{result} {WithResponse(tail, responseSelection)}";
    }

    static int FindMatchingCloseBrace(string s, int openIdx)
    {
        if (openIdx < 0 || openIdx >= s.Length || s[openIdx] != '{') return -1;
        var depth = 0;
        var inString = false;
        var escape = false;
        for (var i = openIdx; i < s.Length; i++)
        {
            var ch = s[i];
            if (inString)
            {
                if (escape) { escape = false; continue; }
                if (ch == '\\') { escape = true; continue; }
                if (ch == '"') inString = false;
                continue;
            }
            if (ch == '"') { inString = true; continue; }
            if (ch == '{') depth++;
            else if (ch == '}')
            {
                depth--;
                if (depth == 0) return i;
            }
        }
        return -1;
    }

    static int LastArgListCloseParen(string s)
    {
        var depth = 0;
        var inString = false;
        var escape = false;
        var last = -1;
        for (var i = 0; i < s.Length; i++)
        {
            var ch = s[i];
            if (inString)
            {
                if (escape) { escape = false; continue; }
                if (ch == '\\') { escape = true; continue; }
                if (ch == '"') inString = false;
                continue;
            }
            if (ch == '"') { inString = true; continue; }
            if (ch == '(') depth++;
            else if (ch == ')')
            {
                depth--;
                if (depth == 0) last = i;
            }
        }
        return last;
    }

    static bool HasTopLevelSelectionBrace(string s)
    {
        var paren = 0;
        var inString = false;
        var escape = false;
        for (var i = 0; i < s.Length; i++)
        {
            var ch = s[i];
            if (inString)
            {
                if (escape) { escape = false; continue; }
                if (ch == '\\') { escape = true; continue; }
                if (ch == '"') inString = false;
                continue;
            }
            if (ch == '"') { inString = true; continue; }
            if (ch == '(') paren++;
            else if (ch == ')') paren--;
            else if (ch == '{' && paren == 0) return true;
        }
        return false;
    }

    static string AppendResponseAfterArgs(string fieldWithArgs, string responseSelection)
    {
        var t = fieldWithArgs.Trim();
        if (t.Contains(responseSelection, StringComparison.Ordinal)) return t;
        var closeParen = LastArgListCloseParen(t);
        if (closeParen < 0) return $"{t} {{ {responseSelection} }}";
        var after = t[(closeParen + 1)..].Trim();
        if (after.StartsWith("{", StringComparison.Ordinal))
        {
            var open = closeParen + 1 + t[(closeParen + 1)..].IndexOf('{');
            var close = FindMatchingCloseBrace(t, open);
            if (close < 0) return $"{t} {{ {responseSelection} }}";
            var head = t[..open].TrimEnd();
            var inner = t[(open + 1)..close].Trim();
            var tail = t[(close + 1)..].Trim();
            return $"{head} {{ {TransformKitSelectionBlock(inner, responseSelection)} }}{(tail.Length == 0 ? "" : " " + tail)}";
        }
        return $"{t[..(closeParen + 1)]} {{ {responseSelection} }}";
    }

    static string TransformKitSelectionBlock(string inner, string responseSelection) =>
        !HasTopLevelSelectionBrace(inner) ? AppendResponseAfterArgs(inner, responseSelection) : WithResponse(inner, responseSelection);
}
