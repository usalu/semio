#nullable enable

namespace Semio;

/// <summary>Replace all fields of an existing <see cref="Kit"/> instance in-place (used by transport sync without computing a <see cref="KitDiff"/>).</summary>
public static class KitState
{
    public static void ReplaceInPlace(Kit target, Kit source)
    {
        target.Id = source.Id;
        target.Name = source.Name;
        target.Version = source.Version;
        target.Description = source.Description;
        target.Icon = source.Icon;
        target.Image = source.Image;
        target.Concepts = source.Concepts;
        target.Tags = source.Tags;
        target.Remote = source.Remote;
        target.Homepage = source.Homepage;
        target.License = source.License;
        target.Authors = source.Authors;
        target.Pieces = source.Pieces;
        target.Groups = source.Groups;
        target.Connections = source.Connections;
        target.Props = source.Props;
        target.Stats = source.Stats;
        target.Attributes = source.Attributes;
        target.Preview = source.Preview;
        target.Qualities = source.Qualities;
        target.Ports = source.Ports;
        target.Files = source.Files;
        target.Folders = source.Folders;
        target.Types = source.Types;
        target.Designs = source.Designs;
        target.CreatedAt = source.CreatedAt;
        target.UpdatedAt = source.UpdatedAt;
    }
}
