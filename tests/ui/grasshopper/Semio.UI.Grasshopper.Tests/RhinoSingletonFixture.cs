using System;
using Xunit;

namespace Rhino.Test
{
    public class RhinoSingletonFixture : IDisposable
  {
    private static bool _isResolverInitialized = false;
    private static RhinoSingletonFixture _instance = null;
    public static RhinoSingletonFixture Instance
    {
      get
      {
        if (null == _instance)
          _instance = new RhinoSingletonFixture();
        return _instance;
      }
    }
    static RhinoSingletonFixture()
    {
      // This MUST be included in a static constructor to ensure that no Rhino DLLs
      // are loaded before the resolver is set up. Avoid creating other static functions
      // and members which may reference Rhino assemblies, as that may cause those
      // assemblies to be loaded before this is called.
      if (!_isResolverInitialized)
      {
        RhinoInside.Resolver.Initialize();
        _isResolverInitialized = true;
      }
    }
    public static void InitializeResolver()
    {
      if (!_isResolverInitialized) RhinoInside.Resolver.Initialize();
      _isResolverInitialized = true;
    }
    private object _Core = null;
    private bool _isDisposed = false;
    private object _docIO { get; set; }
    public RhinoSingletonFixture()
    {
      InitializeCore();
    }
    public Rhino.Runtime.InProcess.RhinoCore Core
    {
      get
      {
        if (null == _Core) InitializeCore();
        return _Core as Rhino.Runtime.InProcess.RhinoCore;
      }
    }

    public void InitializeCore()
    {
      if (null == _Core)
      {
        _Core = new Rhino.Runtime.InProcess.RhinoCore();
      }
    }
    protected virtual void Dispose(bool disposing)
    {
      if (_isDisposed) return;
      if (disposing)
      {
        _docIO = null;
        Core.Dispose();
      }

      // TODO: free unmanaged resources (unmanaged objects) and override finalizer
      // TODO: set large fields to null
      _isDisposed = true;
    }

    // // TODO: override finalizer only if 'Dispose(bool disposing)' has code to free unmanaged resources
    // ~GrasshopperFixture()
    // {
    //     // Do not change this code. Put cleanup code in 'Dispose(bool disposing)' method
    //     Dispose(disposing: false);
    // }

    public void Dispose()
    {
      // Do not change this code. Put cleanup code in 'Dispose(bool disposing)' method
      Dispose(disposing: true);
      GC.SuppressFinalize(this);
    }
  }
}


