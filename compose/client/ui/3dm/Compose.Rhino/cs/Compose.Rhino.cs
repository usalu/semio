#region 📱Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Rhino 8 plugin hosting a WebView2 panel for importing compose kits and representations.

#endregion 📱Header

#region 🔌Adapters
// Host SDK imports (RhinoCommon, WebView2, Newtonsoft) MUST stay in this region.
using System;
using System.Collections.Generic;
using System.Drawing;
using System.Globalization;
using System.IO;
using System.Linq;
using System.Net.Http;
using System.Reflection;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;
#if RHINO_PLUGIN
using System.Windows;
using System.Windows.Controls;
using Rhino.Input.Custom;
#endif
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using Rhino;
using Rhino.Commands;
using Rhino.DocObjects;
using Rhino.Geometry;
using Rhino.PlugIns;
using Rhino.UI;
using Compose;
using RhinoLayer = global::Rhino.DocObjects.Layer;
using Type = Compose.Type;
using File = Compose.File;

#endregion 🔌Adapters

#region 🔌Ports
/// <summary>🦏 Rhino document host port (RhinoCommon adapter in 🔌Adapters).</summary>
public interface IRhinoDocumentHost
{
    RhinoDoc ActiveDoc { get; }
}

/// <summary>🌐 WebView2 host port for embedded React UI.</summary>
public interface IWebViewHost
{
    Task NavigateAsync(string uri);
}
#endregion 🔌Ports

#region ⭐AssemblyAttributes
// Assembly-level attributes required by Rhino to identify this plugin.
// The Id is the plugin ID. PlugInDescription attributes show in Rhino Options > Plug-ins.
#if RHINO_PLUGIN
[assembly: Guid("A1B2C3D4-E5F6-7890-ABCD-EF1234567890")]
[assembly: PlugInDescription(DescriptionType.Address, "")]
[assembly: PlugInDescription(DescriptionType.Country, "")]
[assembly: PlugInDescription(DescriptionType.Email, "ueli@semio-tech.com")]
[assembly: PlugInDescription(DescriptionType.Phone, "")]
[assembly: PlugInDescription(DescriptionType.Organization, "compose")]
[assembly: PlugInDescription(DescriptionType.UpdateUrl, "")]
[assembly: PlugInDescription(DescriptionType.WebSite, "https://compose.tech")]
[assembly: PlugInDescription(DescriptionType.Icon, "Compose.Rhino.compose_32x32.ico")]
#endif
#endregion ⭐AssemblyAttributes

#region ✨Namespace
// Implementations MUST reside in this namespace.
namespace Compose.Rhino;
#endregion ✨Namespace

#region 🎠Constants
// Consumers MUST use these shared constants for configuration.

public static class Constants
{
    public const string Category = Compose.Constants.Name;
    public const string Version = "1.0.0";
    public const string PanelId = "D3A4E2C1-7B8F-4D5E-9A1C-6F2B3E4D5A6B";
}

#endregion 🎠Constants

#region 🔬BridgeProtocol
// Bridge protocol types for JSON-RPC style communication between React UI and C#.

/// <summary>
/// 📨Incoming request from the React UI to native C#.
/// </summary>
public class BridgeRequest
{
    [JsonProperty("id")] public string Id { get; set; } = "";
    [JsonProperty("binding")] public string Binding { get; set; } = "";
    [JsonProperty("method")] public string Method { get; set; } = "";
    [JsonProperty("params")] public JToken? Params { get; set; }
}

/// <summary>
/// 📩Outgoing response from native C# to the React UI.
/// </summary>
public class BridgeResponse
{
    [JsonProperty("id")] public string Id { get; set; } = "";
    [JsonProperty("ok")] public bool Ok { get; set; }
    [JsonProperty("result")] public object? Result { get; set; }
    [JsonProperty("error")] public BridgeError? Error { get; set; }
}

/// <summary>
/// ❌Error detail for a failed bridge response.
/// </summary>
public class BridgeError
{
    [JsonProperty("code")] public string Code { get; set; } = "";
    [JsonProperty("message")] public string Message { get; set; } = "";
    [JsonProperty("details")] public object? Details { get; set; }
}

/// <summary>
/// 📡Outgoing event from native C# to the React UI.
/// </summary>
public class BridgeEvent
{
    [JsonProperty("event")] public string Event { get; set; } = "";
    [JsonProperty("payload")] public object? Payload { get; set; }
}

#endregion 🔬BridgeProtocol

#region ⛹BridgeBinding
// Bridge bindings define the native methods callable from the React UI.

/// <summary>
/// 🔌Interface for a bridge binding exposing named methods.
/// </summary>
public interface IBridgeBinding
{
    string Name { get; }
    IReadOnlyDictionary<string, Func<JToken?, Task<object?>>> Methods { get; }
}

#endregion ⛹BridgeBinding

#region 🪨BridgeRegistry
// Central registry routing bridge requests to the correct binding and method.

/// <summary>
/// 📨Routes incoming bridge requests to registered bindings.
/// </summary>
public class BridgeRegistry
{
    private readonly Dictionary<string, IBridgeBinding> _bindings = new(StringComparer.OrdinalIgnoreCase);

    public void Register(IBridgeBinding binding)
    {
        _bindings[binding.Name] = binding;
    }

    public async Task<BridgeResponse> HandleAsync(BridgeRequest request)
    {
        if (!_bindings.TryGetValue(request.Binding, out var binding))
        {
            return new BridgeResponse
            {
                Id = request.Id,
                Ok = false,
                Error = new BridgeError
                {
                    Code = "BINDING_NOT_FOUND",
                    Message = $"Binding '{request.Binding}' is not registered."
                }
            };
        }

        if (!binding.Methods.TryGetValue(request.Method, out var method))
        {
            return new BridgeResponse
            {
                Id = request.Id,
                Ok = false,
                Error = new BridgeError
                {
                    Code = "METHOD_NOT_FOUND",
                    Message = $"Method '{request.Method}' not found in binding '{request.Binding}'."
                }
            };
        }

        try
        {
            var result = await method(request.Params);
            return new BridgeResponse
            {
                Id = request.Id,
                Ok = true,
                Result = result
            };
        }
        catch (Exception ex)
        {
            return new BridgeResponse
            {
                Id = request.Id,
                Ok = false,
                Error = new BridgeError
                {
                    Code = "INTERNAL_ERROR",
                    Message = ex.Message
                }
            };
        }
    }

    public IEnumerable<string> GetBindingNames() => _bindings.Keys;

    public IEnumerable<string> GetMethodNames(string binding) =>
        _bindings.TryGetValue(binding, out var b) ? b.Methods.Keys : Enumerable.Empty<string>();
}

#endregion 🪨BridgeRegistry

#region 🗻AppBinding
// Application-level bridge binding for version info and diagnostics.

/// <summary>
/// 📌Provides application-level methods: ping, getVersion, getBridgeInfo.
/// </summary>
public class AppBinding : IBridgeBinding
{
    public string Name => "app";

    public IReadOnlyDictionary<string, Func<JToken?, Task<object?>>> Methods => new Dictionary<string, Func<JToken?, Task<object?>>>
    {
        ["ping"] = _ => Task.FromResult<object?>("pong"),
        ["getVersion"] = _ => Task.FromResult<object?>(Constants.Version),
        ["getBridgeInfo"] = _ => Task.FromResult<object?>(new
        {
            protocolVersion = "1.0",
            pluginVersion = Constants.Version,
            rhinoVersion = RhinoApp.Version.ToString()
        })
    };
}

#endregion 🗻AppBinding

#region 📢DocumentBinding
// Document-level bridge binding for Rhino document information.

/// <summary>
/// 🔧Provides document-level methods: getInfo, getUnits, getLayers.
/// </summary>
public class DocumentBinding : IBridgeBinding
{
    public string Name => "document";

    public IReadOnlyDictionary<string, Func<JToken?, Task<object?>>> Methods => new Dictionary<string, Func<JToken?, Task<object?>>>
    {
        ["getInfo"] = _ =>
        {
            var doc = RhinoDoc.ActiveDoc;
            return Task.FromResult<object?>(new
            {
                name = doc?.Name ?? "",
                path = doc?.Path ?? "",
                isModified = doc?.Modified ?? false
            });
        },
        ["getUnits"] = _ =>
        {
            var doc = RhinoDoc.ActiveDoc;
            return Task.FromResult<object?>(new
            {
                system = doc?.ModelUnitSystem.ToString() ?? "None"
            });
        },
        ["getLayers"] = _ =>
        {
            var doc = RhinoDoc.ActiveDoc;
            if (doc == null) return Task.FromResult<object?>(new List<object>());
            var layers = doc.Layers.Select(l => new
            {
                name = l.Name,
                fullPath = l.FullPath,
                id = l.Id.ToString(),
                color = ColorTranslator.ToHtml(l.Color),
                visible = l.IsVisible
            }).ToList();
            return Task.FromResult<object?>(layers);
        }
    };
}

#endregion 📢DocumentBinding

#region 🎢LayerService
// Service for creating and managing Rhino layers following the compose hierarchy.

/// <summary>
/// 🆕Creates nested layer hierarchies for compose imports.
/// Layer path: compose::KITNAME::Types::TYPENAME::Representations::REPRESENTATIONTAGS
/// </summary>
public static class LayerService
{
    /// <summary>
    /// Ensures a layer exists at the given full path, creating parents as needed.
    /// Returns the layer index.
    /// </summary>
    public static int EnsureLayer(RhinoDoc doc, string fullPath)
    {
        var parts = fullPath.Split(new[] { "::" }, StringSplitOptions.RemoveEmptyEntries);
        var parentIndex = -1;

        for (var i = 0; i < parts.Length; i++)
        {
            var layerName = parts[i].Trim();
            var existingIndex = FindLayer(doc, layerName, parentIndex);

            if (existingIndex >= 0)
            {
                parentIndex = existingIndex;
                continue;
            }

            var layer = new RhinoLayer { Name = layerName };

            if (parentIndex >= 0)
                layer.ParentLayerId = doc.Layers[parentIndex].Id;

            parentIndex = doc.Layers.Add(layer);
        }

        return parentIndex;
    }

    /// <summary>
    /// Finds a layer by name under the given parent index.
    /// </summary>
    private static int FindLayer(RhinoDoc doc, string name, int parentIndex)
    {
        for (var i = 0; i < doc.Layers.Count; i++)
        {
            var layer = doc.Layers[i];
            if (!string.Equals(layer.Name, name, StringComparison.OrdinalIgnoreCase))
                continue;

            if (parentIndex < 0 && layer.ParentLayerId == Guid.Empty)
                return i;

            if (parentIndex >= 0 && layer.ParentLayerId == doc.Layers[parentIndex].Id)
                return i;
        }
        return -1;
    }

    /// <summary>
    /// Builds the compose layer path for a representation import.
    /// </summary>
    public static string BuildRepresentationLayerPath(string kitName, string typeName, IEnumerable<string> tags)
    {
        var tagString = string.Join("-", tags.Where(t => !string.IsNullOrEmpty(t)));
        if (string.IsNullOrEmpty(tagString))
            tagString = "default";
        return $"compose::{kitName}::Types::{typeName}::Representations::{tagString}";
    }
}

#endregion 🎢LayerService

#region 🎹ImportBinding
// Bridge binding for importing kits and representations into the active Rhino document.

/// <summary>
/// 📨DTO for an import representation request from the React UI.
/// </summary>
public class ImportRepresentationRequest
{
    [JsonProperty("kitName")] public string KitName { get; set; } = "";
    [JsonProperty("typeName")] public string TypeName { get; set; } = "";
    [JsonProperty("representationId")] public string RepresentationId { get; set; } = "";
    [JsonProperty("fileUrl")] public string FileUrl { get; set; } = "";
    [JsonProperty("tags")] public List<string> Tags { get; set; } = new();
}

/// <summary>
/// 🔧Provides import methods: importRepresentation, importKit (placeholder for file dialog trigger).
/// </summary>
public class ImportBinding : IBridgeBinding
{
    public string Name => "import";

    public IReadOnlyDictionary<string, Func<JToken?, Task<object?>>> Methods => new Dictionary<string, Func<JToken?, Task<object?>>>
    {
        ["importRepresentation"] = async parameters =>
        {
            var request = parameters?.ToObject<ImportRepresentationRequest>()
                ?? throw new ArgumentException("Invalid import representation request.");

            var doc = RhinoDoc.ActiveDoc
                ?? throw new InvalidOperationException("No active Rhino document.");

            var layerPath = LayerService.BuildRepresentationLayerPath(
                request.KitName, request.TypeName, request.Tags);
            var layerIndex = LayerService.EnsureLayer(doc, layerPath);

            if (!string.IsNullOrEmpty(request.FileUrl))
            {
                var tempPath = Path.Combine(Path.GetTempPath(), $"compose_{request.RepresentationId}.3dm");

                try
                {
                    using var client = new HttpClient();
                    var bytes = await client.GetByteArrayAsync(request.FileUrl);
                    System.IO.File.WriteAllBytes(tempPath, bytes);

                    var importedFile = global::Rhino.FileIO.File3dm.Read(tempPath);
                    if (importedFile != null)
                    {
                        foreach (var obj in importedFile.Objects)
                        {
                            if (obj.Geometry == null) continue;
                            var attributes = new ObjectAttributes { LayerIndex = layerIndex };
                            doc.Objects.Add(obj.Geometry, attributes);
                        }
                    }
                }
                finally
                {
                    if (System.IO.File.Exists(tempPath))
                        System.IO.File.Delete(tempPath);
                }
            }

            doc.Views.Redraw();

            return new { layerPath, layerIndex };
        },
        ["openImportKitDialog"] = _ =>
        {
            // Signals the React UI should show its kit import dialog.
            return Task.FromResult<object?>(new { dialogKind = "importKit" });
        }
    };
}

#endregion 🎹ImportBinding

#if RHINO_PLUGIN
#region 🪩ComposeRhinoPlugin
// Main Rhino plugin class bootstrapping the bridge bindings.
// Panel registration is done in the ShowComposeCommand constructor (like Speckle).

/// <summary>
/// 📍Entry point for the compose Rhino plugin.
/// Registers bridge bindings on load. Panel is registered by the command.
/// </summary>
public class ComposeRhinoPlugin : PlugIn
{
    public static ComposeRhinoPlugin? Instance { get; private set; }
    public BridgeRegistry Bridge { get; } = new();
    public ComposeWebViewControl? WebViewControl { get; set; }

    public ComposeRhinoPlugin()
    {
        Instance = this;
    }

    protected override LoadReturnCode OnLoad(ref string errorMessage)
    {
        Bridge.Register(new AppBinding());
        Bridge.Register(new DocumentBinding());
        Bridge.Register(new ImportBinding());

        RhinoApp.WriteLine("compose.3dm plugin loaded.");
        return LoadReturnCode.Success;
    }
}

#endregion 🪩ComposeRhinoPlugin

#region 🔓ComposeWebViewControl
// WPF UserControl hosting WebView2 that loads the compose React UI.
// This control is wrapped by ComposePanelHost (WpfElementHost) for Rhino panel integration.

/// <summary>
/// 👤WPF UserControl hosting WebView2 that loads the compose React UI.
/// Handles bridge message routing between the browser and native C#.
/// </summary>
public class ComposeWebViewControl : System.Windows.Controls.UserControl
{
    private Microsoft.Web.WebView2.Wpf.WebView2? _webView;
    private bool _isReady;

    public ComposeWebViewControl()
    {
        InitializeWebView();
    }

    private async void InitializeWebView()
    {
        _webView = new Microsoft.Web.WebView2.Wpf.WebView2();
        Content = _webView;

        var env = await Microsoft.Web.WebView2.Core.CoreWebView2Environment.CreateAsync(
            null,
            Path.Combine(Path.GetTempPath(), "compose-webview2-data")
        );

        await _webView.EnsureCoreWebView2Async(env);

        _webView.CoreWebView2.Settings.AreDevToolsEnabled =
#if DEBUG
            true;
#else
            false;
#endif
        _webView.CoreWebView2.Settings.IsStatusBarEnabled = false;

        _webView.CoreWebView2.WebMessageReceived += OnWebMessageReceived;

        // Determine URL: dev server or local build
        var uiPath = GetUiPath();
        if (uiPath != null)
            _webView.CoreWebView2.Navigate(uiPath);
        else
            _webView.CoreWebView2.NavigateToString(
                "<html><body><h3>compose UI not found.</h3><p>Run the UI dev server or build the UI.</p></body></html>");

        _isReady = true;
    }

    private string? GetUiPath()
    {
#if DEBUG
        // In debug mode, try the Vite dev server first
        try
        {
            using var client = new System.Net.WebClient();
            client.DownloadString("http://localhost:5174/");
            return "http://localhost:5174/";
        }
        catch
        {
            // Fall through to local build
        }
#endif
        // Look for local built UI assets
        var pluginDir = Path.GetDirectoryName(Assembly.GetExecutingAssembly().Location) ?? "";
        var indexPath = Path.Combine(pluginDir, "ui", "dist", "index.html");
        if (System.IO.File.Exists(indexPath))
            return new Uri(indexPath).AbsoluteUri;

        return null;
    }

    private async void OnWebMessageReceived(object? sender, Microsoft.Web.WebView2.Core.CoreWebView2WebMessageReceivedEventArgs e)
    {
        try
        {
            var json = e.WebMessageAsJson;
            var request = JsonConvert.DeserializeObject<BridgeRequest>(json);
            if (request == null) return;

            var plugin = ComposeRhinoPlugin.Instance;
            if (plugin == null) return;

            var response = await plugin.Bridge.HandleAsync(request);
            var responseJson = JsonConvert.SerializeObject(response);

            _webView?.CoreWebView2?.PostWebMessageAsJson(responseJson);
        }
        catch (Exception ex)
        {
            RhinoApp.WriteLine($"[compose.3dm] Bridge error: {ex.Message}");
        }
    }

    /// <summary>
    /// Sends an event from native C# to the React UI.
    /// </summary>
    public void SendEvent(BridgeEvent evt)
    {
        if (!_isReady || _webView?.CoreWebView2 == null) return;
        var json = JsonConvert.SerializeObject(evt);
        _webView.CoreWebView2.PostWebMessageAsJson(json);
    }
}

#endregion 🔓ComposeWebViewControl

#region 📊ComposePanelHost
// Dockable Rhino panel host wrapping the WPF WebView2 control.
// Follows the Speckle pattern: inherits RhinoWindows.Controls.WpfElementHost.

/// <summary>
/// 👁️WpfElementHost panel wrapping ComposeWebViewControl for Rhino docking.
/// Handles panel close/reopen lifecycle by disconnecting the WPF child.
/// </summary>
[Guid(Constants.PanelId)]
public class ComposePanelHost : RhinoWindows.Controls.WpfElementHost
{
    private readonly ComposeWebViewControl? _webViewControl;

    public ComposePanelHost(uint docSn)
        : base(GetOrCreateWebViewControl(), null)
    {
        _webViewControl = ComposeRhinoPlugin.Instance?.WebViewControl;
        Panels.Closed += PanelsOnClosed;
    }

    private static ComposeWebViewControl GetOrCreateWebViewControl()
    {
        var plugin = ComposeRhinoPlugin.Instance;
        if (plugin?.WebViewControl == null && plugin != null)
        {
            plugin.WebViewControl = new ComposeWebViewControl();
        }
        return plugin?.WebViewControl!;
    }

    /// <summary>
    /// Disconnects the WPF child so the same WebView control can be re-parented when the panel reopens.
    /// </summary>
    public static void Reinitialize(ComposeWebViewControl? webViewControl)
    {
        if (webViewControl == null) return;
        if (Panels.IsPanelVisible(new Guid(Constants.PanelId))) return;
        if (LogicalTreeHelper.GetParent(webViewControl) is Border border)
        {
            border.Child = null;
        }
    }

    private void PanelsOnClosed(object? sender, PanelEventArgs e)
    {
        var composePanelGuid = new Guid(Constants.PanelId);
        if (e.PanelId != composePanelGuid) return;
        if (!Panels.IsPanelVisible(composePanelGuid)) return;

        Panels.Closed -= PanelsOnClosed;

        if (_webViewControl != null)
        {
            if (LogicalTreeHelper.GetParent(_webViewControl) is Border border)
            {
                border.Child = null;
            }
        }
    }
}

#endregion 📊ComposePanelHost

#region 🛒ShowComposeCommand
// Rhino command to open or focus the compose dockable panel.
// Panel registration happens in the constructor (like Speckle connectors).

/// <summary>
/// 📬Command that opens the compose dockable side panel.
/// Registers the panel in its constructor following Speckle's pattern.
/// </summary>
[CommandStyle(global::Rhino.Commands.Style.ScriptRunner)]
public class ShowComposeCommand : Command
{
    public static ShowComposeCommand? Instance { get; private set; }

    public ShowComposeCommand()
    {
        Instance = this;

        var sysIcon = LoadEmbeddedIcon("Compose.Rhino.compose_32x32.ico")
            ?? System.Drawing.SystemIcons.Application;

        Panels.RegisterPanel(
            ComposeRhinoPlugin.Instance!,
            typeof(ComposePanelHost),
            "compose",
            sysIcon,
            PanelType.System
        );
    }

    private static System.Drawing.Icon? LoadEmbeddedIcon(string resourceName)
    {
        var assembly = Assembly.GetExecutingAssembly();
        var stream = assembly.GetManifestResourceStream(resourceName);
        if (stream == null) return null;
        return new System.Drawing.Icon(stream);
    }

    public override string EnglishName => "ShowCompose";

    protected override Result RunCommand(RhinoDoc doc, RunMode mode)
    {
        var panelId = new Guid(Constants.PanelId);

        if (mode == RunMode.Interactive)
        {
            ComposePanelHost.Reinitialize(ComposeRhinoPlugin.Instance?.WebViewControl);
            Panels.OpenPanel(panelId);
            return Result.Success;
        }

        var panelVisible = Panels.IsPanelVisible(panelId);
        var prompt = panelVisible
            ? "compose panel is visible. New value"
            : "compose panel is hidden. New value";

        using var go = new GetOption();
        go.SetCommandPrompt(prompt);
        var hideIndex = go.AddOption("Hide");
        var showIndex = go.AddOption("Show");
        var toggleIndex = go.AddOption("Toggle");
        go.Get();

        if (go.CommandResult() != Result.Success) return go.CommandResult();

        var option = go.Option();
        if (option == null) return Result.Failure;

        var index = option.Index;
        if (index == hideIndex)
        {
            if (panelVisible) Panels.ClosePanel(panelId);
        }
        else if (index == showIndex)
        {
            if (!panelVisible)
            {
                ComposePanelHost.Reinitialize(ComposeRhinoPlugin.Instance?.WebViewControl);
                Panels.OpenPanel(panelId);
            }
        }
        else if (index == toggleIndex)
        {
            if (panelVisible)
            {
                Panels.ClosePanel(panelId);
            }
            else
            {
                ComposePanelHost.Reinitialize(ComposeRhinoPlugin.Instance?.WebViewControl);
                Panels.OpenPanel(panelId);
            }
        }

        return Result.Success;
    }
}

#endregion 🛒ShowComposeCommand
#endif
