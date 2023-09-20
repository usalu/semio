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
    public class LayoutDesignComponent : GH_Component
    {
        private static string _route = "/v1/layoutdesign";
        
        public LayoutDesignComponent() : base("Layout Design", "Layout Design", "Lay out a design.", "Semio", "Server")
        {
        }
        protected override void RegisterInputParams(GH_InputParamManager pManager)
        {
            pManager.AddTextParameter("Url", "Url", "Url of Semio server.", GH_ParamAccess.item,"localhost:2999");
            pManager.AddParameter(new LayoutParam());
            pManager.AddParameter(new PlatformParam());
            pManager[2].Optional = true;
            pManager.AddBooleanParameter("Run", "R", "", GH_ParamAccess.item, false);
        }
        protected override void RegisterOutputParams(GH_OutputParamManager pManager)
        {
            pManager.AddParameter(new DesignParam());
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            string url = "";
            if (!DA.GetData(0, ref url)) return;

            LayoutGoo layout = new();
            if (!DA.GetData(1, ref layout)) return;

            PlatformGoo platform = new();
            DA.GetData(2, ref platform);

            bool run = false;
            if (!DA.GetData(3, ref run)) return;

            if (run)
            {
                var layoutDesignRequest = new LayoutDesignRequest()
                {
                    Layout = layout.Value,
                    TargetPlatform = platform.Value,
                };

                using (var client = new HttpClient()
                       {
                           MaxResponseContentBufferSize = Int32.MaxValue,
                           Timeout = new TimeSpan(0,10,0)

                       })
                {
                    HttpContent content = new StringContent(layoutDesignRequest.ToString());
                    var task = client.PostAsync(url + _route, content);
                    var response = task.Result;
                    if (response.IsSuccessStatusCode)
                    {
                        var designContent = response.Content.ReadAsStringAsync();
                        Design design = Design.Parser.ParseJson(designContent.Result);
                        DA.SetData(0, new DesignGoo(design));
                    }
                    else
                        AddRuntimeMessage(GH_RuntimeMessageLevel.Error,response.ReasonPhrase);
                    
                }
            }
        }
        public override Guid ComponentGuid => new ("15ad0008-1e40-41ff-8e38-116102d7488a");
        protected override System.Drawing.Bitmap Icon => Resources.icon_layoutdesign;
    }
}