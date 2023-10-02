using Grasshopper;
using Grasshopper.Kernel;
using Rhino.Geometry;
using System;
using System.Collections.Generic;
using System.Net.Http;
using Google.Protobuf.Collections;
using Semio.Model.V1;
using Grasshopper.Kernel.Types;
using Semio.Gateway.V1;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Components.Server
{
    public class ClearCacheComponent : GH_Component
    {
        public ClearCacheComponent() : base("Clear Cache", "ClrCache", "Clear the cache of semio.", "semio", "Server")
        {
        }
        protected override void RegisterInputParams(GH_InputParamManager pManager)
        {
            pManager.AddBooleanParameter("Run", "R", "", GH_ParamAccess.item, false);
        }
        protected override void RegisterOutputParams(GH_OutputParamManager pManager)
        {
            
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
      
            bool run = false;
            if (!DA.GetData(0, ref run)) return;

            if (run)
            {
                Backend.clearCache();
            }
        }
        public override Guid ComponentGuid => new ("9a6bd594-4bab-49b8-94df-4cd5d2e111ef");
        protected override System.Drawing.Bitmap Icon => Resources.icon_clearcache;
    }
}