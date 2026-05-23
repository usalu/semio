#nullable enable

using System;
using System.Collections.Generic;
using System.IO;
using Newtonsoft.Json.Linq;

namespace Semio.Store;

/// <summary>📦 WIP {@code theKit.kit} handle: getters for reads, methods for commands, events for field changes (semio/js {@link Kit}).</summary>
public sealed class WipKit
{
    private const string KitObjectSelection =
        "id name description icon image types { edges { node { id name } } } designs { edges { node { id name } } }";

    private readonly StoreSession _session;

    internal WipKit(StoreSession session) => _session = session;

    //#region 📖 getters
    /// <summary>📖 Golden <c>wip.theKit.kit.id</c>.</summary>
    public string Id => GetScalar("id") ?? throw new IOException("graphql: missing kit.id");

    /// <summary>📖 Golden <c>wip.theKit.kit.name</c>.</summary>
    public string Name => GetScalar("name") ?? "";

    /// <summary>📖 Golden <c>wip.theKit.kit.description</c>.</summary>
    public string Description => GetScalar("description") ?? "";

    /// <summary>📖 Materialized WIP branch (<c>initialKit</c>, <c>theKit.kit</c>, checkpoints).</summary>
    public JToken Materialization => _session.ReadMaterialization();

    /// <summary>📖 Full <c>kit { id name … }</c> object under the store cursor.</summary>
    public JObject Object =>
        _session.ReadKitObject(KitObjectSelection) ?? throw new IOException("graphql: missing wip.theKit.kit");
    //#endregion 📖 getters

    //#region 📡 field-change events
    /// <summary>📡 <c>kit.name</c> changed after a successful command.</summary>
    public event Action<string>? NameChanged;

    /// <summary>📡 <c>kit.description</c> changed after a successful command.</summary>
    public event Action<string>? DescriptionChanged;
    //#endregion 📡 field-change events

    //#region 🎬 commands
    /// <summary>🎬 <c>kit.rename</c> on the open unsaved change.</summary>
    public void Rename(string newName) =>
        RunKitCommand($"rename(newName: {StoreGraphql.GqlString(newName)})", "kitRenamed");

    /// <summary>🎬 <c>kit.changeDescription</c>.</summary>
    public void ChangeDescription(string newDescription) =>
        RunKitCommand($"changeDescription(newDescription: {StoreGraphql.GqlString(newDescription)})", "changedDescription");

    public void CreateTag(string name, string? description = null, string? icon = null, int? order = null) =>
        RunKitCommand(
            $"createTag(name: {StoreGraphql.GqlString(name)}, description: {GqlOpt(description)}, icon: {GqlOpt(icon)}, order: {GqlOpt(order)})");

    public void DeleteTag(string id) =>
        RunKitCommand($"deleteTag(id: {StoreGraphql.GqlString(id)})");

    public void DeleteTags(IEnumerable<string> ids) =>
        RunKitCommand($"deleteTags(ids: {StoreGraphql.GqlIdList(ids)})");

    public void CreateConcept(string name, string? description = null, string? icon = null, int? order = null) =>
        RunKitCommand(
            $"createConcept(name: {StoreGraphql.GqlString(name)}, description: {GqlOpt(description)}, icon: {GqlOpt(icon)}, order: {GqlOpt(order)})");

    public void DeleteConcept(string id) =>
        RunKitCommand($"deleteConcept(id: {StoreGraphql.GqlString(id)})");

    public void DeleteConcepts(IEnumerable<string> ids) =>
        RunKitCommand($"deleteConcepts(ids: {StoreGraphql.GqlIdList(ids)})");

    public void CreateQuality(string key, string? value = null, string? unit = null, string? definition = null, string? description = null, string? icon = null) =>
        RunKitCommand(
            $"createQuality(key: {StoreGraphql.GqlString(key)}, value: {GqlOpt(value)}, unit: {GqlOpt(unit)}, definition: {GqlOpt(definition)}, description: {GqlOpt(description)}, icon: {GqlOpt(icon)})");

    public void DeleteQuality(string id) =>
        RunKitCommand($"deleteQuality(id: {StoreGraphql.GqlString(id)})");

    public void DeleteQualities(IEnumerable<string> ids) =>
        RunKitCommand($"deleteQualities(ids: {StoreGraphql.GqlIdList(ids)})");

    public void CreateType(string name, string? description = null, string? icon = null, string? image = null, string? unit = null) =>
        RunKitCommand(
            $"createType(name: {StoreGraphql.GqlString(name)}, description: {GqlOpt(description)}, icon: {GqlOpt(icon)}, image: {GqlOpt(image)}, unit: {GqlOpt(unit)})");

    public void DeleteType(string id) =>
        RunKitCommand($"deleteType(id: {StoreGraphql.GqlString(id)})");

    public void DeleteTypes(IEnumerable<string> ids) =>
        RunKitCommand($"deleteTypes(ids: {StoreGraphql.GqlIdList(ids)})");

    public void CreateDesign(string name, string? description = null, string? icon = null, string? image = null, string? unit = null) =>
        RunKitCommand(
            $"createDesign(name: {StoreGraphql.GqlString(name)}, description: {GqlOpt(description)}, icon: {GqlOpt(icon)}, image: {GqlOpt(image)}, unit: {GqlOpt(unit)})");

    public void DeleteDesign(string id) =>
        RunKitCommand($"deleteDesign(id: {StoreGraphql.GqlString(id)})");

    public void DeleteDesigns(IEnumerable<string> ids) =>
        RunKitCommand($"deleteDesigns(ids: {StoreGraphql.GqlIdList(ids)})");

    /// <summary>🎬 Ensures {@code startNewChange} and returns the change id for subsequent commands.</summary>
    public string EnsureChangeId() => _session.EnsureChangeId();
    //#endregion 🎬 commands

    private string? GetScalar(string field) =>
        StoreGraphqlJson.WipTheKitKitScalar(_session.ReadKitSelection(field), field, _session.StoreId)?.Value<string>();

    private static string GqlOpt(string? s) => s == null ? "null" : StoreGraphql.GqlString(s);

    private static string GqlOpt(int? n) => n == null ? "null" : n.Value.ToString(System.Globalization.CultureInfo.InvariantCulture);

    private void RunKitCommand(string kitSelection, string? fieldEventKind = null)
    {
        var changeId = EnsureChangeId();
        _session.RunKitMutation(changeId, kitSelection, fieldEventKind);
        if (fieldEventKind == "kitRenamed")
        {
            var n = Name;
            NameChanged?.Invoke(n);
        }
        else if (fieldEventKind == "changedDescription")
        {
            var d = Description;
            DescriptionChanged?.Invoke(d);
        }
    }
}
