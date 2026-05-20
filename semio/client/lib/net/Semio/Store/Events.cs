#nullable enable

using System;
using System.Collections.Generic;

namespace Semio.Store;

/// <summary>📡 Command notifications for a {@link StoreSession} (mirrors semio/js {@code EventBus} command kinds).</summary>
public sealed class StoreEventBus
{
    private readonly Dictionary<string, List<Action>> _kindHandlers = new(StringComparer.Ordinal);

    /// <summary>📡 Fires after any kit command mutation succeeds.</summary>
    public event Action? CommandSucceeded;

    public void SubscribeKind(string kind, Action handler)
    {
        if (!_kindHandlers.TryGetValue(kind, out var list))
        {
            list = new List<Action>();
            _kindHandlers[kind] = list;
        }
        if (!list.Contains(handler))
            list.Add(handler);
    }

    public void UnsubscribeKind(string kind, Action handler)
    {
        if (_kindHandlers.TryGetValue(kind, out var list))
            list.Remove(handler);
    }

    internal void PublishKind(string kind)
    {
        if (!_kindHandlers.TryGetValue(kind, out var list)) return;
        foreach (var h in list.ToArray())
            h();
    }

    internal void AfterCommand(string? fieldEventKind = null)
    {
        CommandSucceeded?.Invoke();
        PublishKind("commandSucceeded");
        if (!string.IsNullOrEmpty(fieldEventKind))
            PublishKind(fieldEventKind);
    }
}

/// <summary>📡 Subscribes {@link StoreSession} bus events.</summary>
public static class StoreEventBridge
{
    public static void SubscribeCommandSucceeded(StoreSession session, Action handler) =>
        session.Events.CommandSucceeded += handler;
}
