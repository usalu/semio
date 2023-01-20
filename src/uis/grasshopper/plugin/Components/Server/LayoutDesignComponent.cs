using Grasshopper;
using Grasshopper.Kernel;
using Rhino.Geometry;
using System;
using System.Collections.Generic;
using Google.Protobuf.Collections;
using NetMQ;
using Semio.Assembler.V1;
using Semio.Model.V1;
using NetMQ.Sockets;
using Grasshopper.Kernel.Types;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Components.Server
{
    public class LayoutDesignComponent : GH_Component
    {
        public LayoutDesignComponent() : base("Layout Design", "Layout Design", "Lay out a design.", "Semio", "Server")
        {
        }
        protected override void RegisterInputParams(GH_InputParamManager pManager)
        {
            pManager.AddGenericParameter("Url", "Url", "Url of Semio server.", GH_ParamAccess.item);
            pManager.AddParameter(new LayoutParam());
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
            if (url.StartsWith("https://")) url = url.Substring(8);
            else if (url.StartsWith("http://")) url = url.Substring(7);

            var obj = new GH_ObjectWrapper();
            if (!DA.GetData(1, ref obj)) return;
            if (obj == null) return;
            var layout = (Layout)obj.Value;

            bool run = false;
            if (!DA.GetData(2, ref run)) return;

            string response = "";
            using (var client = new RequestSocket("inproc://" + url))
            {
                var layoutDesignRequest = new LayoutDesignRequest()
                {
                    Layout = layout
                };

                var layoutDesignRequestString = layoutDesignRequest.ToString();
                client.SendFrame(layoutDesignRequestString);
                response = client.ReceiveFrameString();
            }

            DA.SetData(0, response);
        }
        public override Guid ComponentGuid => new ("15ad0008-1e40-41ff-8e38-116102d7488a");
        protected override System.Drawing.Bitmap Icon => Resources.icon_layoutdesign;
    }
}