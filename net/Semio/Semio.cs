#region Header

//Semio.cs
//2020-2025 Ueli Saluz

//This program is free software: you can redistribute it and/or modify
//it under the terms of the GNU Lesser General Public License as
//published by the Free Software Foundation, either version 3 of the
//License, or (at your option) any later version.

//This program is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//GNU Lesser General Public License for more details.

//You should have received a copy of the GNU Lesser General Public License
//along with this program.  If not, see <https://www.gnu.org/licenses/>.

#endregion

#region TODOs

#endregion TODOs

namespace Semio;

#region Domain
public interface IEntity
{
    Guid Id { get; }
    long Version { get; }
    DateTimeOffset? LastModified { get; }
}

public interface IDiffable<TDelta>
{
    TDelta Diff(TDelta? baseState);
    void ApplyDelta(TDelta delta);
}

public interface IValidatable
{
    ValidationResult Validate();
}

public interface IExportable
{
    string ToExportFormat();
}

public interface IImportable<out T>
{
    T FromImportFormat(string data);
}

#region Entities
public interface IKit : IEntity, IDiffable<EntityDelta>, IValidatable
{
    IReadOnlyCollection<IType> Types { get; }
    IReadOnlyCollection<IDesign> Designs { get; }
}

public interface IType : IEntity, IDiffable<EntityDelta>, IValidatable
{
    IReadOnlyCollection<IPort> Ports { get; }
}

public interface IDesign : IEntity, IDiffable<EntityDelta>, IValidatable
{
    IReadOnlyCollection<IPiece> Pieces { get; }
    IReadOnlyCollection<IConnection> Connections { get; }
}

public interface IPiece : IEntity, IDiffable<EntityDelta>, IValidatable
{
    IType Type { get; }
    IDesign Design { get; }
}

public interface IConnection : IEntity, IDiffable<EntityDelta>, IValidatable
{
    IReadOnlyCollection<IPiece> Pieces { get; }  // exactly 2
    IReadOnlyCollection<IPort> Ports { get; }    // exactly 2
}

public interface IPort : IEntity, IDiffable<EntityDelta>, IValidatable
{
}

#endregion Entities

#endregion Domain

#region Core

#region Delta & Sync
public record EntityDelta(
    Guid EntityId,
    long BaseVersion,
    long NewVersion,
    IReadOnlyDictionary<string, object?> Changes,
    IReadOnlyDictionary<string, object?> Tombstones
);

public interface ISynchronizer<TDelta>
{
    IEnumerable<TDelta> CollectOutgoingDeltas();
    SyncResult ApplyIncomingDeltas(IEnumerable<TDelta> deltas);
}

public record SyncResult(
    bool Success,
    IReadOnlyCollection<string> Messages
);

#endregion Delta & Sync

#region Validation
public enum Severity { Info, Warning, Error }

public record ValidationMessage(
    string Code,
    string Field,
    string Message,
    Severity Severity
);

public record ValidationResult(
    IReadOnlyCollection<ValidationMessage> Messages
)
{
    public bool IsValid => !Messages.Any(m => m.Severity == Severity.Error);
}

#endregion Validation

#endregion Core

#region IO

public interface IInputAdapter<TInput, TDomain>
{
    TDomain Parse(TInput input);
}

public interface IOutputAdapter<TDomain, TOutput>
{
    TOutput Render(TDomain domain);
}

#endregion IO

#region Persistence

public interface IRepository<T> where T : IEntity
{
    T? Get(Guid id);
    IEnumerable<T> GetAll();
    void Save(T entity);
    void Delete(Guid id);
}

public interface IDeltaRepository
{
    void SaveDelta(EntityDelta delta);
    IEnumerable<EntityDelta> GetDeltasSince(long version);
}

public interface IExporter
{
    void ExportToSQLite(string filePath);
}

public interface IImporter
{
    void ImportFromSQLite(string filePath);
}

#endregion Persistence